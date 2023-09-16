pub trait Shoe {
    fn deal(&mut self) -> u8;
    fn on_new_round(&mut self) {}
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

pub struct Hand {
    pub cards: Vec<u8>,
    pub status: HandStatus,
    pub value: u8,
    pub soft: bool,
    pub bet_amount: f64,
}

impl Hand {
    fn new(bet_amout: f64) -> Hand {
        Hand {
            cards: Vec::new(),
            status: HandStatus::Value,
            value: 0,
            soft: false,
            bet_amount: bet_amout,
        }
    }

    fn next_hand(&mut self, bet_amount: f64, deal: &mut Box<dyn Shoe>) {
        *self = Hand::new(bet_amount);

        self.deal_card(deal);
        if self.deal_card(deal) == 21 {
            self.status = HandStatus::Blackjack;
        }
    }

    fn split(&mut self, shoe: &mut Box<dyn Shoe>) -> (Hand, u8) {
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

    fn deal_card(&mut self, shoe: &mut Box<dyn Shoe>) -> u8 {
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

    fn can_split(&self) -> bool {
        self.cards.len() == 2 && self.cards[0] == self.cards[1]
    }
}

pub struct Table {
    shoe: Box<dyn Shoe>,
    pub player: Vec<Hand>,
    pub dealer: Hand,
    pub status: RoundStatus,
    pub balance: f64,
}

impl Table {
    pub fn new(shoe: Box<dyn Shoe>) -> Table {
        Table {
            shoe,
            player: vec![Hand::new(0.0)],
            dealer: Hand::new(0.0),
            status: RoundStatus::Concluded,
            balance: 0.0,
        }
    }

    pub fn take_action(&mut self, action: Action) -> Result<(), ()> {
        if let RoundStatus::InProgress(active_hand_index) = self.status {
            match action {
                Action::Stand => {
                    if self.can_take_basic_actions() {
                        Ok(self.next_hand())
                    } else {
                        Result::Err(())
                    }
                }
                Action::Hit => {
                    if self.can_take_basic_actions() {
                        if (&mut self.player[active_hand_index]).deal_card(&mut self.shoe) >= 21 {
                            self.next_hand();
                        }
                        Result::Ok(())
                    } else {
                        Result::Err(())
                    }
                }
                Action::Double => {
                    if self.can_double() {
                        self.balance -= (&mut self.player[active_hand_index]).bet_amount;
                        (&mut self.player[active_hand_index]).bet_amount *= 2.0;
                        self.take_action(Action::Hit)
                    } else {
                        Result::Err(())
                    }
                }
                Action::Split => {
                    if self.can_split() {
                        self.balance -= (&mut self.player[active_hand_index]).bet_amount;
                        let (new_hand, old_hand_value) =
                            (&mut self.player[active_hand_index]).split(&mut self.shoe);
                        self.player.insert(active_hand_index + 1, new_hand);
                        if old_hand_value >= 21 {
                            self.next_hand()
                        }
                        Result::Ok(())
                    } else {
                        Result::Err(())
                    }
                }
                Action::Surrender => {
                    if self.can_surrender() {
                        self.balance += (&mut self.player[active_hand_index]).bet_amount / 2.0;
                        (&mut self.player[active_hand_index]).status = HandStatus::Surrender;
                        Ok(self.next_hand())
                    } else {
                        Result::Err(())
                    }
                }
                Action::Deal(_) => todo!("can't deal right now"),
            }
        } else {
            if let Action::Deal(bet_amount) = action {
                self.shoe.on_new_round();
                self.balance -= bet_amount;
                self.dealer.next_hand(0.0, &mut self.shoe);
                self.player[0].next_hand(bet_amount, &mut self.shoe);
                self.player.truncate(1);

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
                Result::Ok(())
            } else {
                Result::Err(())
            }
        }
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
        if self
            .player
            .iter()
            .all(|hand| hand.status == HandStatus::Bust || hand.status == HandStatus::Surrender)
        {
            return;
        }
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
