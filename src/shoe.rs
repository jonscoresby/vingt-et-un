use rand::seq::SliceRandom;

pub trait Shoe {
    fn deal(&mut self) -> u8;
    fn on_new_round(&mut self) {}
}

pub struct StandardShoe {
    deck: Vec<u8>,
    size: u8,
}

impl StandardShoe {
    pub fn new(size: u8) -> Box<StandardShoe> {
        Box::new(Self::create_shoe(size))
    }

    fn create_shoe(size: u8) -> StandardShoe {
        let mut deck = Vec::new();
        (0..size * 4).for_each(|_| {
            deck.extend([1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 10, 10, 10]);
        });
        deck.shuffle(&mut rand::thread_rng());

        StandardShoe { deck, size }
    }
}

impl Shoe for StandardShoe {
    fn deal(&mut self) -> u8 {
        match self.deck.pop(){
            None => {
                self.on_new_round();
                self.deal()
            },
            Some(x) => x
        }
    }

    fn on_new_round(&mut self) {
        if self.deck.len() < 20 {
            *self = Self::create_shoe(self.size);
        }
    }
}

pub(crate) struct CustomShoe{
    pub(crate) deck: Vec<u8>
}

impl CustomShoe {
    pub(crate) fn new(deck: Vec<u8>) -> Box<dyn Shoe>{
        Box::new(CustomShoe{
            deck
        })
    }
}

impl Shoe for CustomShoe{
    fn deal(&mut self) -> u8 {
        self.deck.pop().unwrap()
    }
}

#[cfg(test)]
mod shoe_tests {
    use super::*;

    #[test]
    fn standard_shoe_creation() {
        let shoe = StandardShoe::new(8);
        assert_eq!(shoe.deck.iter().filter(|x| **x == 10).count(), 128);
        assert_eq!(shoe.deck.iter().filter(|x| **x == 3).count(), 32);
    }

    #[test]
    fn standard_shoe_no_cards() {
        let mut shoe = StandardShoe::new(1);
        shoe.deck.clear();
        shoe.deal();
    }
}
