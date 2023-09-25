use crate::hand::{Hand, HandStatus, PlayerTrait, Position};
use crate::shoe::Shoe;
use crate::table::RoundStatus::InProgress;
use crate::HandStatus::Completed;
use crate::PossibleAction;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(PartialEq, Debug)]
pub enum RoundStatus {
    Concluded,
    InProgress(usize),
}

pub(crate) fn calculate_round_start(dealer: &Hand, positions: &mut Vec<Position>) {
    for position in positions {
        match (dealer.status, position.hand.status) {
            (HandStatus::Blackjack, HandStatus::Blackjack) => {
                *(&mut position.player.borrow_mut()).balance() += position.bet_amount;
                position.hand.status = HandStatus::Push;
            }
            (HandStatus::Blackjack, _) => position.hand.status = HandStatus::Lose,
            (_, HandStatus::Blackjack) => {
                *(&mut position.player.borrow_mut()).balance() += position.bet_amount * 5.0 / 2.0
            }
            (_, _) => {}
        }
    }
}

pub struct Game<'a> {
    shoe: Box<dyn Shoe>,
    get_action: fn(&Game, Vec<PossibleAction>) -> PossibleAction,
    pub positions: Vec<Position<'a>>,
    pub dealer: Hand,
    pub status: RoundStatus,
}
impl<'a, 'b> Game<'a> {
    pub fn new(
        shoe: Box<dyn Shoe>,
        get_action: fn(&Game, Vec<PossibleAction>) -> PossibleAction,
    ) -> Game<'a> {
        Game {
            shoe,
            get_action,
            positions: vec![],
            status: RoundStatus::Concluded,
            dealer: Hand {
                cards: vec![],
                status: Completed,
                value: 0,
                soft: false,
            },
        }
    }

    pub fn create_position(
        &mut self,
        player: &'b mut dyn PlayerTrait,
        bet_amount: f64,
    ) -> Result<Position<'b>, ()> {
        if *player.balance() > bet_amount {
            Ok(Position {
                hand: Hand::new(&mut self.shoe),
                player: Rc::new(RefCell::new(player)),
                bet_amount,
            })
        } else {
            Err(())
        }
    }

    pub fn play_round(&mut self, positions: Vec<Position<'a>>){
        self.positions = positions;
        self.dealer = Hand::new(&mut self.shoe);
        calculate_round_start(&self.dealer, &mut self.positions);
        self.status = update_round_status(&self.positions, 0);

        loop {
            if let InProgress(active_position_index) = self.status {
                let active_position = &mut self.positions[active_position_index];
                let possible_actions = active_position.get_possible_actions();
                let action = (self.get_action)(self, possible_actions).action();
                if let Some(position) = self.positions[active_position_index].take_action(action, &mut self.shoe) {
                    self.positions.insert(active_position_index + 1, position)
                }
                update_round_status(&self.positions, active_position_index);
            } else {
                break;
            }
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
        update_round_status(positions, i + 1)
    } else {
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
        *(&mut position.player.borrow_mut()).balance() += match position.hand.status {
            HandStatus::Win => position.bet_amount * 2.0,
            HandStatus::Push => position.bet_amount,
            _ => 0.0,
        };
    }
}
