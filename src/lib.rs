use rand::seq::SliceRandom;

fn create_shoe(size: u8) -> Vec<u8> {
    let mut vec = Vec::new();
    (0..size * 4).for_each(|_| {
        vec.extend([1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 10, 10, 10]);
    });
    vec.shuffle(&mut rand::thread_rng());
    vec
}

#[derive(PartialEq, Copy, Clone)]
pub enum Action {
    Deal(f64),
    Hit,
    Stand,
    Double,
    Split,
    Surrender,
}

#[derive(PartialEq, Debug)]
pub enum RoundStatus {
    Concluded,
    InProgress(usize),
}

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

#[derive(Clone)]
pub struct Hand {
    pub cards: Vec<u8>,
    pub status: HandStatus,
    pub value: u8,
    pub bet_amount: f64,
}

impl Hand {
    fn new(bet_amout: f64) -> Hand {
        Hand {
            cards: Vec::new(),
            status: HandStatus::Value,
            value: 0,
            bet_amount: bet_amout,
        }
    }

    fn split(&mut self, shoe: &mut Vec<u8>) -> (Hand, u8) {
        let mut new_hand = Hand {
            cards: vec![self.cards.pop().unwrap()],
            status: HandStatus::Value,
            value: 0,
            bet_amount: self.bet_amount,
        };
        self.deal_card(shoe);
        new_hand.deal_card(shoe);
        (new_hand, self.value)
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

    fn can_split(&self) -> bool {
        self.cards.len() == 2 && self.cards[0] == self.cards[1]
    }
}

pub struct Table {
    shoe: Vec<u8>,
    pub player: Vec<Hand>,
    pub dealer: Hand,
    pub status: RoundStatus,
    pub balance: f64,
}

impl Table {
    pub fn new(balance: f64) -> Table {
        Table {
            shoe: Vec::new(),
            player: vec![Hand::new(0.0)],
            dealer: Hand::new(0.0),
            status: RoundStatus::Concluded,
            balance,
        }
    }

    pub fn take_action(&mut self, action: Action) {
        if let RoundStatus::InProgress(active_hand_index) = self.status {
            let can_surrender = self.can_surrender();
            let active_hand = &mut self.player[active_hand_index];
            match action {
                Action::Stand => self.next_hand(),
                Action::Hit => {
                    if active_hand.deal_card(&mut self.shoe) >= 21 {
                        self.next_hand();
                    }
                }
                Action::Double => {
                    self.balance -= active_hand.bet_amount;
                    active_hand.bet_amount *= 2.0;
                    self.take_action(Action::Hit);
                }
                Action::Split => {
                    if active_hand.can_split() {
                        self.balance -= active_hand.bet_amount;
                        let (new_hand, old_hand_value) = active_hand.split(&mut self.shoe);
                        self.player.insert(active_hand_index + 1, new_hand);
                        if old_hand_value >= 21 { self.next_hand()}
                    } else {
                        todo!("can't split right now")
                    }
                }
                Action::Surrender => {
                    if can_surrender {
                        self.balance += active_hand.bet_amount / 2.0;
                        active_hand.status = HandStatus::Surrender;
                        self.next_hand();
                    } else {
                        todo!("can't surrender")
                    }
                }
                Action::Deal(_) => todo!("can't deal right now"),
            }
        } else {
            if let Action::Deal(bet_amount) = action {
                self.balance -= bet_amount;
                if self.shoe.len() < 20 {
                    self.shoe = create_shoe(4);
                }
                self.dealer = self.new_hand(0.0);
                self.player = vec![self.new_hand(bet_amount)];

                self.player[0].status = match (&self.player[0].status, &self.dealer.status) {
                    (HandStatus::Blackjack, HandStatus::Blackjack) => {
                        self.balance += self.player[0].bet_amount;
                        self.dealer.status = HandStatus::Push;
                        HandStatus::Push
                    }
                    (HandStatus::Blackjack, _) => {
                        self.balance += self.player[0].bet_amount * 5.0 / 2.0;
                        HandStatus::Blackjack
                    }
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

    fn new_hand(&mut self, bet_amount: f64) -> Hand {
        let mut hand = Hand::new(bet_amount);

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
        if self.player.iter().all(|hand| hand.status == HandStatus::Bust || hand.status == HandStatus::Surrender) {return;}
        while self.dealer.value < 17 {
            self.dealer.deal_card(&mut self.shoe);
        }
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
            self.balance += match hand.status {
                HandStatus::Win => hand.bet_amount * 2.0,
                HandStatus::Push => hand.bet_amount,
                _ => 0.0,
            }
        }
    }

    pub fn can_split(&self) -> bool {
        if let RoundStatus::InProgress(i) = self.status {
            self.player[i].can_split()
        } else {
            false
        }
    }

    pub fn can_surrender(&self) -> bool {
        self.player.len() == 1 && self.player[0].cards.len() == 2 && self.can_take_basic_actions()
    }

    pub fn can_double(&self) -> bool {
        self.can_take_basic_actions()
    }

    pub fn can_deal(&self) -> bool {
        self.status == RoundStatus::Concluded
    }

    pub fn can_take_basic_actions(&self) -> bool {
        !self.can_deal()
    }
}
