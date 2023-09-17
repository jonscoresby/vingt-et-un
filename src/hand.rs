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

pub struct Hand {
    pub cards: Vec<u8>,
    pub status: HandStatus,
    pub value: u8,
    pub soft: bool,
    pub bet_amount: f64,
}

impl Hand {
    pub(crate) fn new(bet_amount: f64) -> Hand {
        Hand {
            cards: Vec::new(),
            status: HandStatus::Value,
            value: 0,
            soft: false,
            bet_amount,
        }
    }

    pub(crate) fn next_hand(&mut self, bet_amount: f64, deal: &mut Box<dyn Shoe>) {
        *self = Hand::new(bet_amount);

        self.deal_card(deal);
        if self.deal_card(deal) == 21 {
            self.status = HandStatus::Blackjack;
        }
    }

    pub(crate) fn split(&mut self, shoe: &mut Box<dyn Shoe>) -> (Hand, u8) {
        let mut new_hand = Hand {
            cards: vec![self.cards.pop().unwrap()],
            status: HandStatus::Value,
            value: 0,
            soft: false,
            bet_amount: self.bet_amount,
        };
        self.deal_card(shoe);
        new_hand.deal_card(shoe);
        (new_hand, self.value)
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

    pub(crate) fn can_split(&self) -> bool {
        self.cards.len() == 2 && self.cards[0] == self.cards[1]
    }
}
