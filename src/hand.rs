use std::cell::RefCell;
use std::rc::Rc;
use crate::shoe::Shoe;

#[derive(PartialEq, PartialOrd, Copy, Clone)]
pub enum HandStatus {
    Value,
    Bust,
    Win,
    Push,
    Lose,
    Blackjack,
    Surrender,
}

pub trait PlayerTrait{
    fn balance(&self) -> f64;
    fn add_amount(&mut self, amount: f64);
}

#[derive(PartialEq)]
pub struct Player {
    balance: f64,
}

impl Player {
    pub fn new() -> Player {
        Player{
            balance: 0.0,
        }
    }
}

impl PlayerTrait for Player{
    fn balance(&self) -> f64 {
        self.balance
    }
    fn add_amount(&mut self, amount: f64) {
        self.balance += amount;
    }
}

pub struct Position<'a> {
    pub hand: Hand,
    pub player: Rc<RefCell<&'a dyn PlayerTrait>>,
    pub bet_amount: f64,
}
impl Position<'_> {
    //probably remove shoe from this function
    pub fn new<'a>(shoe: &mut Box<dyn Shoe>, player: &'a dyn PlayerTrait, bet_amount: f64) -> Result<Position<'a>, ()>{
        if player.balance() > bet_amount {
            Ok(Position{
                hand: Hand::new(shoe),
                player: Rc::new(RefCell::new(player)),
                bet_amount,
            })
        }else {
            Err(())
        }
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
            player: self.player.clone()
        };
        self.hand.deal_card(shoe);
        new_hand.hand.deal_card(shoe);
        new_hand
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

        self.value
    }

    fn dealer_turn(&mut self, shoe: &mut Box<dyn Shoe>){
        while self.value < 17 {
            self.deal_card(shoe);
        }
    }

    pub(crate) fn can_split(&self) -> bool {
        self.cards.len() == 2 && self.cards[0] == self.cards[1]
    }
}
