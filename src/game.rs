use crate::game::PlayerBalanceError::{
    BalanceCannotBeNegative, HandDoesNotExist, PlayerDoesNotExist,
};
use crate::hand::Hand;
use crate::player_hand::PlayerHand;
use crate::round::Round;
use crate::shoe::Shoe;
use crate::HandStatus::{Stood, Value};
use crate::PossibleAction;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
pub enum PlayerBalanceError {
    PlayerDoesNotExist,
    HandDoesNotExist,
    BalanceCannotBeNegative,
}

pub struct Game {
    shoe: Box<dyn Shoe>,
    get_action: fn(&Round, Vec<PossibleAction>) -> PossibleAction,
    player_balances: Vec<Rc<RefCell<f64>>>,
    player_bets: Vec<Vec<f64>>,
}

impl Game {
    pub fn start_game(
        shoe: Box<dyn Shoe>,
        new_round: fn(&mut Game, &Round),
        get_action: fn(&Round, Vec<PossibleAction>) -> PossibleAction,
    ) {
        let mut game = Game {
            shoe,
            get_action,
            player_balances: vec![],
            player_bets: vec![],
        };

        let mut round = Round {
            player_hands: vec![],
            active_hand_index: 0,
            dealer: Hand {
                cards: vec![],
                status: Value,
                value: 0,
                soft: false,
            },
        };

        loop {
            game.reset_bets();
            new_round(&mut game, &round);
            round = game.play_round();
            game.shoe.new_round()
        }
    }

    pub fn get_player_balances(&self) -> Vec<f64> {
        self.player_balances.iter().map(|x| *x.borrow()).collect()
    }

    pub fn set_player_balances(
        &mut self,
        player_balances: Vec<f64>,
    ) -> Result<(), PlayerBalanceError> {
        if player_balances.iter().any(|x| *x < 0.0) {
            Err(BalanceCannotBeNegative)
        } else {
            self.player_balances = player_balances
                .iter()
                .map(|x| Rc::from(RefCell::from(*x)))
                .collect();
            self.reset_bets();
            Ok(())
        }
    }

    pub fn get_player_balance(&self, player_index: usize) -> Result<f64, PlayerBalanceError> {
        match self.player_balances.get(player_index) {
            None => Err(PlayerDoesNotExist),
            Some(x) => Ok(*x.borrow()),
        }
    }

    pub fn set_player_balance(
        &mut self,
        player_index: usize,
        new_balance: f64,
    ) -> Result<(), PlayerBalanceError> {
        match self.player_balances.get(player_index) {
            None => Err(PlayerDoesNotExist),
            Some(x) => {
                if new_balance < 0.0 {
                    Err(BalanceCannotBeNegative)
                } else {
                    *x.borrow_mut() = new_balance;
                    self.reset_bet(player_index);
                    Ok(())
                }
            }
        }
    }

    pub fn modify_player_balance(
        &mut self,
        player_index: usize,
        new_balance: f64,
    ) -> Result<(), PlayerBalanceError> {
        match self.player_balances.get(player_index) {
            None => Err(PlayerDoesNotExist),
            Some(x) => {
                if -new_balance > *x.borrow() {
                    Err(BalanceCannotBeNegative)
                } else {
                    *x.borrow_mut() += new_balance;
                    self.reset_bet(player_index);
                    Ok(())
                }
            }
        }
    }

    pub fn reset_bets(&mut self) {
        for i in 0..self.player_balances.len() {
            self.reset_bet(i)
        }
    }

    fn reset_bet(&mut self, index: usize) {
        match self.player_bets.get(index) {
            None => self.player_bets.insert(index, vec![0.0]),
            Some(x) => {
                if x.iter().sum::<f64>() > *self.player_balances[index].borrow() {
                    self.player_bets[index] = vec![0.0];
                }
            }
        }
    }

    pub fn get_bet(
        &self,
        player_index: usize,
        hand_index: usize,
    ) -> Result<f64, PlayerBalanceError> {
        match self.player_balances.get(player_index) {
            None => Err(PlayerDoesNotExist),
            Some(_) => match self.player_bets[player_index].get(hand_index) {
                None => Err(HandDoesNotExist),
                Some(x) => Ok(*x),
            },
        }
    }

    pub fn set_bet(
        &mut self,
        player_index: usize,
        hand_index: usize,
        amount: f64,
    ) -> Result<(), PlayerBalanceError> {
        self.get_bet(player_index, hand_index)?;

        self.modify_player_balance(player_index, -amount)?;

        self.player_bets[player_index][hand_index] = amount;
        Ok(())
    }

    pub fn add_bet(&mut self, player_index: usize, amount: f64) -> Result<(), PlayerBalanceError> {
        if player_index >= self.player_balances.len() {
            return Err(PlayerDoesNotExist);
        }

        self.modify_player_balance(player_index, -amount)?;
        self.player_bets[player_index].push(amount);
        Ok(())
    }

    fn create_player_hands(&mut self) -> Vec<PlayerHand> {
        let mut player_hands = Vec::<PlayerHand>::new();
        for (i, x) in self.player_bets.iter().enumerate() {
            for y in x {
                player_hands.push(PlayerHand {
                    hand: Hand::new(&mut self.shoe),
                    player_balance: self.player_balances[i].clone(),
                    bet_amount: *y,
                    split: false,
                })
            }
        }
        player_hands
    }

    fn play_round(&mut self) -> Round {
        let mut round = Round {
            player_hands: self.create_player_hands(),
            dealer: Hand::new(&mut self.shoe),
            active_hand_index: 0,
        };

        round.start();

        while round.update_active_hand_index() {
            let active_player_hand = &mut round.player_hands[round.active_hand_index];
            let possible_actions = active_player_hand.get_possible_actions();
            let action = (self.get_action)(&round, possible_actions).action();
            if let Some(player_hand) =
                round.player_hands[round.active_hand_index].take_action(action, &mut self.shoe)
            {
                round
                    .player_hands
                    .insert(round.active_hand_index + 1, player_hand)
            }
        }

        if round
            .player_hands
            .iter()
            .any(|player_hand| player_hand.hand.status == Stood)
        {
            round.dealer.dealer_turn(&mut self.shoe);
            round.end();
        }

        round
    }
}
