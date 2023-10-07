use crate::shoe::Shoe;

#[derive(PartialEq, PartialOrd, Copy, Clone, Debug)]
pub enum HandStatus {
    Value,
    Bust,
    Win,
    Push,
    Lose,
    Blackjack,
    Surrender,
    Stood,
}

#[derive(PartialEq, Debug)]
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
        hand.deal_card(shoe);
        hand
    }

    pub(crate) fn deal_card(&mut self, shoe: &mut Box<dyn Shoe>) -> u8 {
        self.cards.push(shoe.deal());
        self.calculate_value();
        self.value
    }

    pub(crate) fn calculate_value(&mut self){
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
            self.status = HandStatus::Blackjack;

            if self.cards.len() > 2 {
                self.status = HandStatus::Stood
            }
        }
    }

    pub(crate) fn dealer_turn(&mut self, shoe: &mut Box<dyn Shoe>) {
        while self.value < 17 {
            self.deal_card(shoe);
        }
    }
}

#[cfg(test)]
mod hand_tests {
    use super::*;
    use crate::shoe::CustomShoe;

    #[test]
    fn hand_value_calculation() {
        let mut shoe = CustomShoe::new(vec![7, 7, 7, 7]);

        let mut hand = Hand {
            cards: vec![],
            status: HandStatus::Value,
            value: 0,
            soft: false,
        };

        assert_eq!(hand.deal_card(&mut shoe), 7);
        assert_eq!(hand.status, HandStatus::Value);

        assert_eq!(hand.deal_card(&mut shoe), 14);
        assert_eq!(hand.status, HandStatus::Value);

        assert_eq!(hand.deal_card(&mut shoe), 21);
        assert_eq!(hand.status, HandStatus::Stood);

        assert_eq!(hand.deal_card(&mut shoe), 28);
        assert_eq!(hand.status, HandStatus::Bust);
    }

    #[test]
    fn soft_hand_value_calculation() {
        let mut shoe = CustomShoe::new(vec![1, 9, 1, 1, 7]);

        let mut hand = Hand {
            cards: vec![],
            status: HandStatus::Value,
            value: 0,
            soft: false,
        };

        assert_eq!(hand.deal_card(&mut shoe), 7);
        assert!(!hand.soft);

        assert_eq!(hand.deal_card(&mut shoe), 18);
        assert!(hand.soft);

        assert_eq!(hand.deal_card(&mut shoe), 19);
        assert!(hand.soft);

        assert_eq!(hand.deal_card(&mut shoe), 18);
        assert!(!hand.soft);

        assert_eq!(hand.deal_card(&mut shoe), 19);
        assert!(!hand.soft);
    }

    #[test]
    fn new_hand_blackjack() {
        let mut shoe = CustomShoe::new(vec![10, 1]);

        let hand = Hand::new(&mut shoe);
        assert_eq!(hand.status, HandStatus::Blackjack);
    }
    #[test]
    fn hand_hit_twenty_one() {
        let mut shoe = CustomShoe::new(vec![8, 8, 5]);

        let mut hand = Hand::new(&mut shoe);
        hand.deal_card(&mut shoe);
        assert_eq!(hand.status, HandStatus::Stood);
    }
}
