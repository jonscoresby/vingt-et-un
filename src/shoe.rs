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
        self.deck.pop().unwrap()
    }

    fn on_new_round(&mut self) {
        if self.deck.len() < 20 {
            *self = Self::create_shoe(self.size);
        }
    }
}

