use crate::position::Position;
use crate::Hand;
use crate::HandStatus::{Blackjack, Stood, Lose, Push, Surrender, Value, Win};

pub struct Round {
    pub positions: Vec<Position>,
    pub active_position_index: usize,
    pub dealer: Hand,
}

impl Round {
    pub(crate) fn calculate_round_start(&mut self) {
        for position in &mut self.positions {
            match (self.dealer.status, position.hand.status) {
                (Blackjack, Blackjack) => {
                    *position.player.borrow_mut() += position.bet_amount;
                    position.hand.status = Push;
                }
                (Blackjack, _) => position.hand.status = Lose,
                (_, Blackjack) => *position.player.borrow_mut() += position.bet_amount * 5.0 / 2.0,
                (_, _) => {}
            }
        }
    }
    pub(crate) fn update_round_status(&mut self) -> bool {
        if self.active_position_index == self.positions.len() {
            false
        } else if self.positions[self.active_position_index].hand.value == 21
            || self.positions[self.active_position_index].hand.status == Stood
            || self.positions[self.active_position_index].hand.status == Surrender
        {
            self.active_position_index += 1;
            self.update_round_status()
        } else {
            true
        }
    }

    pub(crate) fn calculate_round_end(&mut self) {
        for position in &mut self.positions {
            if let Value = position.hand.status {
                position.hand.status = if let Value = self.dealer.status {
                    match position.hand.value.cmp(&self.dealer.value) {
                        std::cmp::Ordering::Less => Lose,
                        std::cmp::Ordering::Equal => Push,
                        std::cmp::Ordering::Greater => Win,
                    }
                } else {
                    Win
                }
            }

            *position.player.borrow_mut() += match position.hand.status {
                Win => position.bet_amount * 2.0,
                Push => position.bet_amount,
                _ => 0.0,
            };
        }
    }
}

#[cfg(test)]
mod round_tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
