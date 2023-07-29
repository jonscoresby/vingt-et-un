use rand::seq::SliceRandom;

fn create_shoe(size: u8) -> Vec<u8> {
    let mut vec = Vec::new();
    (0..size * 4).for_each(|_| {
        vec.extend([1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 10, 10, 10]);
    });
    vec.shuffle(&mut rand::thread_rng());
    vec
}

#[derive(Eq, PartialEq, Hash)]
pub enum Action {
    Hit,
    Stand,
    Deal,
    Split,
}

#[derive(PartialEq, PartialOrd, Copy, Clone)]
pub enum HandStatus {
    Value,
    Bust,
    Win,
    Push,
    Lose,
    Blackjack,
}

#[derive(PartialEq, Debug)]
pub enum RoundStatus {
    Concluded,
    InProgress(usize),
}

pub struct Hand {
    pub cards: Vec<u8>,
    pub status: HandStatus,
    pub value: u8,
}

impl Hand {
    fn new() -> Hand {
        Hand {
            cards: Vec::new(),
            status: HandStatus::Value,
            value: 0,
        }
    }

    fn split(&mut self, shoe: &mut Vec<u8>) -> Hand {
        let mut new_hand = Hand {
            cards: vec![self.cards.pop().unwrap()],
            status: HandStatus::Value,
            value: 0,
        };
        self.deal_card(shoe);
        new_hand.deal_card(shoe);
        new_hand
    }

    fn deal_card(&mut self, shoe: &mut Vec<u8>) -> u8 {
        self.cards.push(shoe.pop().unwrap());

        let mut aces: u8 = self
            .cards
            .iter()
            .filter(|&n| *n == 1)
            .count()
            .try_into()
            .unwrap();
        self.value = aces * 10 + self.cards.iter().sum::<u8>();

        while self.value > 21 && aces > 0 {
            self.value -= 10;
            aces -= 1;
        }

        if self.value > 21 {
            self.status = HandStatus::Bust
        }

        self.value
    }

    fn is_splittable(&self) -> bool {
        self.cards.len() == 2 && self.cards[0] == self.cards[1]
    }
}

pub struct Table {
    shoe: Vec<u8>,
    pub player: Vec<Hand>,
    pub dealer: Hand,
    pub status: RoundStatus,
}

impl Table {
    pub fn new() -> Table {
        let mut game = Table {
            shoe: Vec::new(),
            player: vec![Hand::new()],
            dealer: Hand::new(),
            status: RoundStatus::Concluded,
        };
        game.take_action(Action::Deal);
        game
    }

    pub fn take_action(&mut self, action: Action) {
        if let RoundStatus::InProgress(active_hand_index) = self.status {
            let active_hand = &mut self.player[active_hand_index];
            match action {
                Action::Stand => self.next_hand(),
                Action::Hit => {
                    if active_hand.deal_card(&mut self.shoe) >= 21 {
                        self.next_hand();
                    }
                }
                Action::Split => {
                    if active_hand.is_splittable() {
                        let new_hand = active_hand.split(&mut self.shoe);
                        self.player.insert(active_hand_index + 1, new_hand);
                    } else {
                        todo!("can't split right now")
                    }
                }
                Action::Deal => todo!("can't deal right now"),
            }
        } else {
            if let Action::Deal = action {
                if self.shoe.len() < 20 {
                    self.shoe = create_shoe(4);
                }
                self.dealer = self.new_hand();
                self.player = vec![self.new_hand()];

                self.player[0].status = match (&self.player[0].status, &self.dealer.status) {
                    (HandStatus::Blackjack, HandStatus::Blackjack) => {
                        self.dealer.status = HandStatus::Push;
                        HandStatus::Push
                    }
                    (HandStatus::Blackjack, _) => HandStatus::Blackjack,
                    (_, HandStatus::Blackjack) => HandStatus::Lose,
                    _ => {
                        self.status = RoundStatus::InProgress(0);
                        self.player[0].status
                    }
                };
            } else {
                todo!("invalid action. start game with deal");
            }
        }
    }

    fn new_hand(&mut self) -> Hand {
        let mut hand = Hand::new();

        hand.deal_card(&mut self.shoe);
        if hand.deal_card(&mut self.shoe) == 21 {
            hand.status = HandStatus::Blackjack
        }

        hand
    }

    fn next_hand(&mut self) {
        if let RoundStatus::InProgress(i) = self.status {
            self.status = self.get_new_round_status(i + 1);
        }
    }

    fn get_new_round_status(&mut self, i: usize) -> RoundStatus {
        if i == self.player.len() {
            self.dealer_turn();
            RoundStatus::Concluded
        } else if self.player[i].value == 21 {
            self.get_new_round_status(i + 1)
        } else {
            RoundStatus::InProgress(i)
        }
    }

    fn dealer_turn(&mut self) {
        while self.dealer.value < 17 {self.dealer.deal_card(&mut self.shoe);}
        for hand in &mut self.player {
            if let HandStatus::Value = hand.status {
                hand.status = if let HandStatus::Value = self.dealer.status {
                    match hand.value.cmp(&self.dealer.value) {
                        std::cmp::Ordering::Less => HandStatus::Lose,
                        std::cmp::Ordering::Equal => HandStatus::Push,
                        std::cmp::Ordering::Greater => HandStatus::Win,
                    }
                } else {
                    HandStatus::Win
                }
            }
        }
    }
}
