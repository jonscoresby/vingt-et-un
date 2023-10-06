use crate::hand::{Hand};
use crate::position::Position;
use crate::round::Round;
use crate::shoe::Shoe;
use crate::table::PlayerBalanceError::{BalanceCannotBeNegative, PlayerDoesNotExist};
use crate::PossibleAction;
use std::cell::RefCell;
use std::rc::Rc;
use crate::HandStatus::Stood;

#[derive(Debug)]
pub enum PlayerBalanceError {
    PlayerDoesNotExist,
    BalanceCannotBeNegative,
}

pub struct Game {
    shoe: Box<dyn Shoe>,
    get_action: fn(&Round, Vec<PossibleAction>) -> PossibleAction,
    balances: Vec<Rc<RefCell<f64>>>,
}

impl Game {
    pub fn new(
        shoe: Box<dyn Shoe>,
        get_action: fn(&Round, Vec<PossibleAction>) -> PossibleAction,
        player_balances: Vec<f64>,
    ) -> Game {
        Game {
            shoe,
            get_action,
            balances: player_balances
                .iter()
                .map(|x| Rc::from(RefCell::from(*x)))
                .collect(),
        }
    }

    pub fn get_player_balances(&self) -> Vec<f64> {
        self.balances.iter().map(|x| *x.borrow()).collect()
    }

    pub fn update_player_balances(
        &mut self,
        player_balances: Vec<f64>,
    ) -> Result<(), PlayerBalanceError> {
        if player_balances.iter().any(|x| *x < 0.0) {
            Err(BalanceCannotBeNegative)
        } else {
            self.balances = player_balances
                .iter()
                .map(|x| Rc::from(RefCell::from(*x)))
                .collect();
            Ok(())
        }
    }

    pub fn get_player_balance(&self, player_index: usize) -> Result<f64, PlayerBalanceError> {
        match self.balances.get(player_index) {
            None => Err(PlayerDoesNotExist),
            Some(x) => Ok(*x.borrow()),
        }
    }

    pub fn set_player_balance(
        &self,
        player_index: usize,
        new_balance: f64,
    ) -> Result<(), PlayerBalanceError> {
        match self.balances.get(player_index) {
            None => Err(PlayerDoesNotExist),
            Some(x) => {
                if new_balance < 0.0 {
                    Err(BalanceCannotBeNegative)
                } else {
                    *x.borrow_mut() = new_balance;
                    Ok(())
                }
            }
        }
    }

    pub fn modify_player_balance(
        &self,
        player_index: usize,
        new_balance: f64,
    ) -> Result<(), PlayerBalanceError> {
        match self.balances.get(player_index) {
            None => Err(PlayerDoesNotExist),
            Some(x) => {
                if -new_balance > *x.borrow() {
                    Err(BalanceCannotBeNegative)
                } else {
                    *x.borrow_mut() += new_balance;
                    Ok(())
                }
            }
        }
    }

    pub fn create_position(
        &mut self,
        player_index: usize,
        bet_amount: f64,
    ) -> Result<Position, PlayerBalanceError> {
        let balance = match self.balances.get(player_index) {
            None => return Err(PlayerDoesNotExist),
            Some(x) => x,
        };

        if *balance.borrow() < bet_amount {
            return Err(BalanceCannotBeNegative);
        }

        Ok(Position {
            hand: Hand::new(&mut self.shoe),
            player: balance.clone(),
            bet_amount,
            can_surrender: true,
        })
    }

    pub fn play_round(&mut self, positions: Vec<Position>) -> Round {
        let mut round = Round {
            positions,
            dealer: Hand::new(&mut self.shoe),
            active_position_index: 0,
        };

        round.calculate_round_start();

        while round.update_round_status() {
            let active_position = &mut round.positions[round.active_position_index];
            let possible_actions = active_position.get_possible_actions();
            let action = (self.get_action)(&round, possible_actions).action();
            if let Some(position) = round.positions[round.active_position_index].take_action(action, &mut self.shoe) {
                round.positions.insert(round.active_position_index + 1, position)
            }
        }

        if round.positions.iter().any(|position| position.hand.status == Stood) {
            round.dealer.dealer_turn(&mut self.shoe);
            round.calculate_round_end();
        }

        round
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
