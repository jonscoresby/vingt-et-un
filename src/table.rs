use crate::hand::{Hand, HandStatus};
use crate::shoe::Shoe;
use crate::table::RoundStatus::InProgress;
use crate::HandStatus::{Completed, Value};
use crate::PossibleAction;
use std::cell::RefCell;
use std::rc::Rc;
use crate::position::Position;
use crate::table::PlayerBalanceError::{BalanceCannotBeNegative, PlayerDoesNotExist};

#[derive(PartialEq, Debug)]
pub enum RoundStatus {
    Concluded,
    InProgress(usize),
}

pub(crate) fn calculate_round_start(dealer: &Hand, positions: &mut Vec<Position>) {
    for position in positions {
        match (dealer.status, position.hand.status) {
            (HandStatus::Blackjack, HandStatus::Blackjack) => {
                *position.player.borrow_mut() += position.bet_amount;
                position.hand.status = HandStatus::Push;
            }
            (HandStatus::Blackjack, _) => position.hand.status = HandStatus::Lose,
            (_, HandStatus::Blackjack) => {
                *position.player.borrow_mut() += position.bet_amount * 5.0 / 2.0
            }
            (_, _) => {}
        }
    }
}

#[derive(Debug)]
pub enum PlayerBalanceError{
    PlayerDoesNotExist,
    BalanceCannotBeNegative
}

pub struct Game {
    shoe: Box<dyn Shoe>,
    get_action: fn(&Game, Vec<PossibleAction>) -> PossibleAction,
    balances: Vec<Rc<RefCell<f64>>>,
    pub positions: Vec<Position>,
    pub dealer: Hand,
    pub status: RoundStatus,
}

impl Game {
    pub fn new(
        shoe: Box<dyn Shoe>,
        get_action: fn(&Game, Vec<PossibleAction>) -> PossibleAction,
        player_balances: Vec<f64>
    ) -> Game {
        Game {
            shoe,
            get_action,
            balances: player_balances.iter().map(|x| Rc::from(RefCell::from(*x))).collect(),
            positions: vec![],
            status: RoundStatus::Concluded,
            dealer: Hand {
                cards: vec![],
                status: Value,
                value: 0,
                soft: false,
            },
        }
    }

    pub fn get_player_balances(&self) -> Vec<f64>{
        self.balances.iter().map(|x| *x.borrow()).collect()
    }

    pub fn update_player_balances(&mut self, player_balances: Vec<f64>) -> Result<(), PlayerBalanceError>{
        if player_balances.iter().any(|x| *x < 0.0) {Err(BalanceCannotBeNegative) }
        else {
            self.balances = player_balances.iter().map(|x| Rc::from(RefCell::from(*x))).collect();
            Ok(())
        }
    }

    pub fn get_player_balance(&self, player_index: usize) -> Result<f64, PlayerBalanceError> {
        match self.balances.get(player_index){
            None => Err(PlayerDoesNotExist),
            Some(x) => Ok(*x.borrow())
        }
    }

    pub fn set_player_balance(&mut self, player_index: usize, new_balance: f64) -> Result<(), PlayerBalanceError> {
        match self.balances.get(player_index){
            None => Err(PlayerDoesNotExist),
            Some(x) => {
                if new_balance < 0.0 {Err(BalanceCannotBeNegative)}
                else{
                    *x.borrow_mut() = new_balance;
                    Ok(())
                }
            }
        }
    }

    pub fn modify_player_balance(&mut self, player_index: usize, new_balance: f64) -> Result<(), PlayerBalanceError> {
        match self.balances.get(player_index){
            None => Err(PlayerDoesNotExist),
            Some(x) => {
                if -new_balance > *x.borrow() {Err(BalanceCannotBeNegative)}
                else{
                    *x.borrow_mut() += new_balance;
                    Ok(())
                }
            }
        }
    }

    pub fn create_position(&mut self, player_index: usize, bet_amount: f64,) -> Result<Position, PlayerBalanceError> {
        let balance = match self.balances.get(player_index){
            None => return Err(PlayerDoesNotExist),
            Some(x) => x,
        };
        if *balance.borrow() < bet_amount { return Err(BalanceCannotBeNegative)}
        Ok(Position {
            hand: Hand::new(&mut self.shoe),
            player: balance.clone(),
            bet_amount,
            can_surrender: true,
        })
    }

    pub fn play_round(&mut self, positions: Vec<Position>){
        self.positions = positions;
        self.dealer = Hand::new(&mut self.shoe);
        calculate_round_start(&self.dealer, &mut self.positions);
        self.status = update_round_status(&self.positions, 0);

            while let InProgress(active_position_index) = self.status {
                let active_position = &mut self.positions[active_position_index];
                let possible_actions = active_position.get_possible_actions();
                let action = (self.get_action)(self, possible_actions).action();
                if let Some(position) = self.positions[active_position_index].take_action(action, &mut self.shoe) {
                    self.positions.insert(active_position_index + 1, position)
                }
                self.status = update_round_status(&self.positions, active_position_index);
            }

        if self.positions.iter().all(|position| {
            position.hand.status == HandStatus::Bust
                || position.hand.status == HandStatus::Surrender
        }) {
            return;
        }
        self.dealer.dealer_turn(&mut self.shoe);
        calculate_round_end(&self.dealer, &mut self.positions);
    }
}

fn update_round_status(positions: &Vec<Position>, i: usize) -> RoundStatus {
    if i == positions.len() {
        RoundStatus::Concluded
    } else if positions[i].hand.value == 21
        || positions[i].hand.status == Completed
        || positions[i].hand.status == HandStatus::Surrender
    {
        println!("Complete { }", i);
        update_round_status(positions, i + 1)
    } else {
        println!("continue");
        InProgress(i)
    }
}

fn calculate_round_end(dealer: &Hand, positions: &mut Vec<Position>) {
    for position in positions {
        if let HandStatus::Value = position.hand.status {
            position.hand.status = if let HandStatus::Value = dealer.status {
                match position.hand.value.cmp(&dealer.value) {
                    std::cmp::Ordering::Less => HandStatus::Lose,
                    std::cmp::Ordering::Equal => HandStatus::Push,
                    std::cmp::Ordering::Greater => HandStatus::Win,
                }
            } else {
                HandStatus::Win
            }
        }
        *position.player.borrow_mut() += match position.hand.status {
            HandStatus::Win => position.bet_amount * 2.0,
            HandStatus::Push => position.bet_amount,
            _ => 0.0,
        };
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
