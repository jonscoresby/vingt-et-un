use crate::shoe::Shoe;
use crate::Action::{Double, Hit, Split, Stand, Surrender};
use crate::HandStatus::Completed;
use crate::{Action, PossibleAction};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(PartialEq, PartialOrd, Copy, Clone)]
pub enum HandStatus {
    Completed,
    Value,
    Bust,
    Win,
    Push,
    Lose,
    Blackjack,
    Surrender,
}

pub trait PlayerTrait {
    fn balance(&mut self) -> &mut f64;
}

#[derive(PartialEq)]
pub struct Player {
    balance: f64,
}

impl Player {
    pub fn new() -> Player {
        Player { balance: 0.0 }
    }
}

impl PlayerTrait for Player {
    fn balance(&mut self) -> &mut f64 {
        &mut self.balance
    }
}

pub struct Position<'a> {
    pub hand: Hand,
    pub player: Rc<RefCell<&'a mut dyn PlayerTrait>>,
    pub bet_amount: f64,
}
impl<'a> Position<'a> {
    pub(crate) fn take_action(
        &mut self,
        action: Action,
        shoe: &mut Box<dyn Shoe>,
    ) -> Option<Position<'a>> {
        match action {
            Stand => self.hand.status = Completed,
            Hit => {
                self.hand.deal_card(shoe);
            }
            Double => {
                *(&mut self.player.borrow_mut()).balance() -= self.bet_amount;
                self.bet_amount *= 2.0;
                self.hand.deal_card(shoe);
            }
            Split => {
                *(&mut self.player.borrow_mut()).balance() -= self.bet_amount;
                return Some(self.split(shoe));
            }
            Surrender => {
                *(&mut self.player.borrow_mut()).balance() += self.bet_amount / 2.0;
                self.hand.status = HandStatus::Surrender;
            }
        };
        return None;
    }

    pub(crate) fn split(&mut self, shoe: &mut Box<dyn Shoe>) -> Position<'a> {
        let mut new_hand = Position {
            hand: Hand {
                cards: vec![self.hand.cards.pop().unwrap()],
                status: HandStatus::Value,
                value: 0,
                soft: false,
            },
            bet_amount: self.bet_amount,
            player: self.player.clone(),
        };
        self.hand.deal_card(shoe);
        new_hand.hand.deal_card(shoe);
        new_hand
    }

    pub(crate) fn get_possible_actions(&self) -> Vec<PossibleAction> {
        let mut possible_actions: Vec<PossibleAction> = Vec::new();

        possible_actions.push(PossibleAction(Hit));
        possible_actions.push(PossibleAction(Stand));
        possible_actions.push(PossibleAction(Double));

        if self.hand.cards.len() == 2 && self.hand.cards[0] == self.hand.cards[1] {
            possible_actions.push(PossibleAction(Split));
        }
        if Rc::strong_count(&self.player) == 1 && self.hand.cards.len() == 2 {
            possible_actions.push(PossibleAction(Surrender));
        }

        possible_actions
    }
}

#[derive(PartialEq)]
pub struct Hand {
    pub cards: Vec<u8>,
    pub status: HandStatus,
    pub value: u8,
    pub soft: bool,
}

impl Hand {
    pub(crate) fn new(shoe: &mut Box<dyn Shoe>) -> Hand {
        let mut hand = Hand {
            cards: Vec::new(),
            status: HandStatus::Value,
            value: 0,
            soft: false,
        };

        hand.deal_card(shoe);
        if hand.deal_card(shoe) == 21 {
            hand.status = HandStatus::Blackjack;
        };

        hand
    }

    pub(crate) fn deal_card(&mut self, shoe: &mut Box<dyn Shoe>) -> u8 {
        self.cards.push(shoe.deal());

        let aces = self.cards.iter().filter(|&n| *n == 1).count();
        self.value = self.cards.iter().sum::<u8>();

        self.soft = self.value < 12 && aces > 0;
        if self.soft {
            self.value += 10;
        }

        if self.value > 21 {
            self.status = HandStatus::Bust
        }

        if self.value == 21 {
            self.status = Completed
        }

        self.value
    }

    pub(crate) fn dealer_turn(&mut self, shoe: &mut Box<dyn Shoe>) {
        while self.value < 17 {
            self.deal_card(shoe);
        }
    }
}
