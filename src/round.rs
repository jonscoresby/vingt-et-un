use crate::player_hand::PlayerHand;
use crate::Hand;
use crate::HandStatus::{Blackjack, Lose, Push, Stood, Value, Win};

pub struct Round {
    pub player_hands: Vec<PlayerHand>,
    pub active_hand_index: usize,
    pub dealer: Hand,
}

impl Round {
    pub(crate) fn start(&mut self) {
        for player_hand in &mut self.player_hands {
            match (self.dealer.status, player_hand.hand.status) {
                (Blackjack, Blackjack) => {
                    *player_hand.player_balance.borrow_mut() += player_hand.bet_amount;
                    player_hand.hand.status = Push;
                }
                (Blackjack, _) => player_hand.hand.status = Lose,
                (_, Blackjack) => {
                    *player_hand.player_balance.borrow_mut() += player_hand.bet_amount * 5.0 / 2.0
                }
                (_, _) => {}
            }
        }
    }
    pub(crate) fn update_active_hand_index(&mut self) -> bool {
        if self.active_hand_index == self.player_hands.len() {
            false
        } else if self.player_hands[self.active_hand_index].hand.value == 21
            || self.player_hands[self.active_hand_index].hand.status != Value
        {
            self.active_hand_index += 1;
            self.update_active_hand_index()
        } else {
            true
        }
    }

    pub(crate) fn end(&mut self) {
        for player_hand in &mut self.player_hands {
            if let Stood = player_hand.hand.status {
                player_hand.hand.status = if let Value = self.dealer.status {
                    match player_hand.hand.value.cmp(&self.dealer.value) {
                        std::cmp::Ordering::Less => Lose,
                        std::cmp::Ordering::Equal => Push,
                        std::cmp::Ordering::Greater => Win,
                    }
                } else {
                    Win
                }
            }

            *player_hand.player_balance.borrow_mut() += match player_hand.hand.status {
                Win => player_hand.bet_amount * 2.0,
                Push => player_hand.bet_amount,
                _ => 0.0,
            };
        }
    }
}

#[cfg(test)]
mod round_tests {
    use crate::player_hand::PlayerHand;
    use crate::round::Round;
    use crate::shoe::{CustomShoe, Shoe};
    use crate::Hand;
    use crate::HandStatus::{Blackjack, Lose, Push, Stood, Value, Win};
    use std::cell::RefCell;
    use std::rc::Rc;

    fn test_round(shoe: &mut Box<dyn Shoe>) -> Round {
        Round {
            player_hands: vec![
                PlayerHand {
                    hand: Hand::new(shoe),
                    bet_amount: 4.0,
                    player_balance: Rc::new(RefCell::new(10.0)),
                    split: false,
                },
                PlayerHand {
                    hand: Hand::new(shoe),
                    bet_amount: 4.0,
                    player_balance: Rc::new(RefCell::new(10.0)),
                    split: false,
                },
            ],
            active_hand_index: 0,
            dealer: Hand::new(shoe),
        }
    }

    #[test]
    fn test_round_start_dealer_blackjack() {
        let mut shoe: Box<dyn Shoe> = CustomShoe::new(vec![10, 1, 1, 10, 8, 8]);
        let mut round = test_round(&mut shoe);
        round.start();
        assert_eq!(Lose, round.player_hands[0].hand.status);
        assert!(*round.player_hands[0].player_balance.borrow() < 10.01);

        assert_eq!(Push, round.player_hands[1].hand.status);
        assert!(*round.player_hands[1].player_balance.borrow() > 13.99);
    }

    #[test]
    fn test_round_start_no_dealer_blackjack() {
        let mut shoe: Box<dyn Shoe> = CustomShoe::new(vec![8, 8, 1, 10, 8, 8]);
        let mut round = test_round(&mut shoe);
        round.start();

        assert_eq!(Value, round.player_hands[0].hand.status);
        assert!(*round.player_hands[0].player_balance.borrow() < 10.01);

        assert_eq!(Blackjack, round.player_hands[1].hand.status);
        assert!(*round.player_hands[1].player_balance.borrow() > 19.99);
    }

    #[test]
    fn test_round_update_second_active() {
        let mut shoe: Box<dyn Shoe> = CustomShoe::new(vec![7, 8, 8, 8, 8, 7, 7]);
        let mut round = test_round(&mut shoe);
        round.player_hands[0].hand.deal_card(&mut shoe);

        assert!(round.update_active_hand_index());
        assert_eq!(round.active_hand_index, 1);

        round.player_hands[1].hand.status = Stood;
        assert!(!round.update_active_hand_index());
    }

    #[test]
    fn test_round_end() {
        let mut shoe: Box<dyn Shoe> = CustomShoe::new(vec![1, 5, 8, 10, 8, 8, 7, 7]);
        let mut round = test_round(&mut shoe);
        round.player_hands[0].hand.deal_card(&mut shoe);
        round.player_hands[1].hand.deal_card(&mut shoe);
        round.player_hands[0].hand.status = Stood;
        round.player_hands[1].hand.status = Stood;

        round.end();

        assert_eq!(Win, round.player_hands[0].hand.status);
        assert!(*round.player_hands[0].player_balance.borrow() > 13.99);

        assert_eq!(Lose, round.player_hands[1].hand.status);
        assert!(*round.player_hands[1].player_balance.borrow() < 10.01);
    }
}
