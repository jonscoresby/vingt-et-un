use std::cell::RefCell;
use std::rc::Rc;
use crate::{Action, Hand, HandStatus, PossibleAction};
use crate::Action::{Double, Hit, Split, Stand, Surrender};
use crate::HandStatus::Completed;
use crate::shoe::Shoe;

#[derive(Debug)]
pub struct Position {
    pub hand: Hand,
    pub player: Rc<RefCell<f64>>,
    pub bet_amount: f64,
    pub can_surrender: bool,
}
impl Position {
    pub(crate) fn take_action(
        &mut self,
        action: Action,
        shoe: &mut Box<dyn Shoe>,
    ) -> Option<Position> {
        match action {
            Stand => self.hand.status = Completed,
            Hit => { self.hand.deal_card(shoe); }
            Double => self.double(shoe),
            Split => return Some(self.split(shoe)),
            Surrender => self.surrender(),
        };
        None
    }

    fn double(&mut self, shoe: &mut Box<dyn Shoe>){
        *self.player.borrow_mut() -= self.bet_amount;
        self.bet_amount *= 2.0;
        self.hand.deal_card(shoe);
    }

    pub(crate) fn split(&mut self, shoe: &mut Box<dyn Shoe>) -> Position {
        let mut new_hand = Position {
            hand: Hand {
                cards: vec![self.hand.cards.pop().unwrap()],
                status: HandStatus::Value,
                value: 0,
                soft: false,
            },
            bet_amount: self.bet_amount,
            player: self.player.clone(),
            can_surrender: false,
        };
        *self.player.borrow_mut() -= self.bet_amount;
        self.can_surrender = false;
        self.hand.deal_card(shoe);
        new_hand.hand.deal_card(shoe);
        new_hand
    }

    fn surrender(&mut self){
        *self.player.borrow_mut() += self.bet_amount / 2.0;
        self.hand.status = HandStatus::Surrender;
    }

    pub(crate) fn get_possible_actions(&self) -> Vec<PossibleAction> {
        let mut possible_actions: Vec<PossibleAction> = Vec::new();

        possible_actions.push(PossibleAction(Hit));
        possible_actions.push(PossibleAction(Stand));

        if self.bet_amount <= *self.player.borrow(){
            possible_actions.push(PossibleAction(Double));
        }

        if self.hand.cards.len() == 2 && self.hand.cards[0] == self.hand.cards[1] && self.bet_amount <= *self.player.borrow() {
            possible_actions.push(PossibleAction(Split));
        }
        if self.can_surrender && self.hand.cards.len() == 2 {
            possible_actions.push(PossibleAction(Surrender));
        }

        possible_actions
    }
}

#[cfg(test)]
mod position_tests {
    use std::cell::RefCell;
    use std::rc::Rc;
    use crate::{Action, Hand, PossibleAction};
    use crate::position::Position;
    use crate::shoe::{CustomShoe, Shoe, StandardShoe};

    fn test_position(shoe: &mut Box<dyn Shoe>, bet_amount: f64) -> Position {
        let balance = Rc::from(RefCell::from(100.00));
        Position {
            hand: Hand::new(shoe),
            player: balance.clone(),
            bet_amount,
            can_surrender: true
        }
    }

    #[test]
    fn double_possible_action() {
        let mut shoe: Box<dyn Shoe> = StandardShoe::new(1);
        let mut position = test_position(&mut shoe, 60.0);

        assert!(position.get_possible_actions().contains(&PossibleAction(Action::Double)));
        position.double(&mut shoe);
        assert!(!position.get_possible_actions().contains(&PossibleAction(Action::Double)));
    }

    #[test]
    fn surrender_possible_action() {
        let mut shoe: Box<dyn Shoe> = StandardShoe::new(1);
        let mut position = test_position(&mut shoe, 0.0);
        let position2 = position.split(&mut shoe);

        assert!(!position.get_possible_actions().contains(&PossibleAction(Action::Surrender)));
        assert!(!position2.get_possible_actions().contains(&PossibleAction(Action::Surrender)));
    }

    #[test]
    fn split_possible_action() {
        let mut shoe = CustomShoe::new(vec![2, 2, 8, 8, 8, 8]);
        let mut position = test_position(&mut shoe, 40.0);

        assert!(position.get_possible_actions().contains(&PossibleAction(Action::Split)));

        let position2 = position.split(&mut shoe);
        assert!(position.get_possible_actions().contains(&PossibleAction(Action::Split)));
        assert!(position2.get_possible_actions().contains(&PossibleAction(Action::Split)));

        let position3 = position.split(&mut shoe);
        assert!(!position.get_possible_actions().contains(&PossibleAction(Action::Split)));
        println!("{:?}", position2);
        assert!(!position2.get_possible_actions().contains(&PossibleAction(Action::Split)));
        assert!(!position3.get_possible_actions().contains(&PossibleAction(Action::Split)));
    }
}
