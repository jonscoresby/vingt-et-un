use crate::shoe::Shoe;
use crate::Action::{Double, Hit, Split, Stand, Surrender};
use crate::{Action, Hand, HandStatus, PossibleAction};
use std::cell::RefCell;
use std::rc::Rc;
use crate::HandStatus::Stood;

#[derive(Debug)]
pub struct PlayerHand {
    pub hand: Hand,
    pub bet_amount: f64,
    pub(crate) player_balance: Rc<RefCell<f64>>,
    pub(crate) split: bool,
}

impl PlayerHand {
    pub(crate) fn take_action(
        &mut self,
        action: Action,
        shoe: &mut Box<dyn Shoe>,
    ) -> Option<PlayerHand> {
        match action {
            Stand => self.hand.status = Stood,
            Hit => {
                self.hand.deal_card(shoe);
            }
            Double => self.double(shoe),
            Split => return Some(self.split(shoe)),
            Surrender => self.surrender(),
        };
        None
    }

    fn double(&mut self, shoe: &mut Box<dyn Shoe>) {
        *self.player_balance.borrow_mut() -= self.bet_amount;
        self.bet_amount *= 2.0;
        self.hand.deal_card(shoe);
    }

    pub(crate) fn split(&mut self, shoe: &mut Box<dyn Shoe>) -> PlayerHand {
        let mut new_hand = PlayerHand {
            hand: Hand {
                cards: vec![self.hand.cards.pop().unwrap()],
                status: HandStatus::Value,
                value: 0,
                soft: false,
            },
            bet_amount: self.bet_amount,
            player_balance: self.player_balance.clone(),
            split: true,
        };
        *self.player_balance.borrow_mut() -= self.bet_amount;
        self.split = true;
        self.hand.deal_card(shoe);
        new_hand.hand.deal_card(shoe);
        new_hand
    }

    fn surrender(&mut self) {
        *self.player_balance.borrow_mut() += self.bet_amount / 2.0;
        self.hand.status = HandStatus::Surrender;
    }

    pub(crate) fn get_possible_actions(&self) -> Vec<PossibleAction> {
        let mut possible_actions: Vec<PossibleAction> = Vec::new();

        possible_actions.push(PossibleAction(Hit));
        possible_actions.push(PossibleAction(Stand));

        if self.bet_amount <= *self.player_balance.borrow() {
            possible_actions.push(PossibleAction(Double));
        }

        if self.hand.cards.len() == 2
            && self.hand.cards[0] == self.hand.cards[1]
            && self.bet_amount <= *self.player_balance.borrow()
        {
            possible_actions.push(PossibleAction(Split));
        }
        if !self.split && self.hand.cards.len() == 2 {
            possible_actions.push(PossibleAction(Surrender));
        }

        possible_actions
    }

    pub fn balance(&self) -> f64 {
        *self.player_balance.borrow()
    }

}

#[cfg(test)]
mod player_hand_tests {
    use crate::player_hand::PlayerHand;
    use crate::shoe::{CustomShoe, Shoe, StandardShoe};
    use crate::{Hand, PossibleAction};
    use std::cell::RefCell;
    use std::rc::Rc;
    use crate::Action::{Double, Split, Surrender};

    fn test_player_hand(shoe: &mut Box<dyn Shoe>, bet_amount: f64) -> PlayerHand {
        let balance = Rc::from(RefCell::from(100.00));
        PlayerHand {
            hand: Hand::new(shoe),
            player_balance: balance.clone(),
            bet_amount,
            split: false
        }
    }

    #[test]
    fn double_possible_action() {
        let mut shoe: Box<dyn Shoe> = StandardShoe::new(1);

        let mut player_hand = test_player_hand(&mut shoe, 60.0);
        assert!(player_hand.get_possible_actions().contains(&PossibleAction(Double)));

        player_hand.double(&mut shoe);
        assert!(!player_hand.get_possible_actions().contains(&PossibleAction(Double)));
    }

    #[test]
    fn surrender_possible_action() {
        let mut shoe: Box<dyn Shoe> = StandardShoe::new(1);

        let mut player_hand = test_player_hand(&mut shoe, 0.0);
        assert!(player_hand.get_possible_actions().contains(&PossibleAction(Surrender)));

        let player_hand2 = player_hand.split(&mut shoe);
        assert!(!player_hand.get_possible_actions().contains(&PossibleAction(Surrender)));
        assert!(!player_hand2.get_possible_actions().contains(&PossibleAction(Surrender)));
    }

    #[test]
    fn split_possible_action() {
        let mut shoe = CustomShoe::new(vec![2, 2, 8, 8, 8, 8]);
        let mut player_hand = test_player_hand(&mut shoe, 40.0);
        assert!(player_hand.get_possible_actions().contains(&PossibleAction(Split)));

        let player_hand2 = player_hand.split(&mut shoe);
        assert!(player_hand.get_possible_actions().contains(&PossibleAction(Split)));
        assert!(player_hand2.get_possible_actions().contains(&PossibleAction(Split)));

        let player_hand3 = player_hand.split(&mut shoe);
        assert!(!player_hand.get_possible_actions().contains(&PossibleAction(Split)));
        assert!(!player_hand2.get_possible_actions().contains(&PossibleAction(Split)));
        assert!(!player_hand3.get_possible_actions().contains(&PossibleAction(Split)));
    }
}
