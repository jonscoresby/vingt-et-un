use rand::seq::SliceRandom;
use std::collections::HashSet;

fn create_shoe(size: u8) -> Vec<u8> {
    let mut vec = Vec::new();
    (0..size * 4).for_each(|_| {
        vec.extend([1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 10, 10, 10]);
    });
    vec.shuffle(&mut rand::thread_rng());
    vec
}

pub struct Player {
    pub hand: Vec<u8>,
    status: HandStatus,
}

impl Player {
    fn deal_new_hand(&mut self, shoe: &mut Vec<u8>) {
        self.hand.clear();
        self.deal_card(shoe);
        self.deal_card(shoe);
    }

    fn new() -> Player {
        Player {
            hand: Vec::new(),
            status: HandStatus::Value(0),
        }
    }

    fn deal_card(&mut self, shoe: &mut Vec<u8>) -> &HandStatus {
        self.hand.push(shoe.pop().unwrap());

        let mut aces: u8 = self
            .hand
            .iter()
            .filter(|&n| *n == 1)
            .count()
            .try_into()
            .unwrap();
        let mut base_sum = aces * 10 + self.hand.iter().sum::<u8>();

        while base_sum > 21 && aces > 0 {
            base_sum -= 10;
            aces -= 1;
        }
        self.status = if base_sum > 21 {
            HandStatus::Bust
        } else {
            HandStatus::Value(base_sum)
        };
        &self.status
    }

    fn is_blackjack(&self) -> bool {
        self.status == HandStatus::Value(21)
    }
}

#[derive(PartialEq, PartialOrd)]
enum HandStatus {
    Value(u8),
    Bust,
}

#[derive(PartialEq, Debug)]
pub enum RoundStatus {
    Concluded(RoundOutcome),
    InProgress,
}

#[derive(PartialEq, Debug)]
pub enum RoundOutcome {
    Win,
    Lose,
    Push,
    Blackjack,
}

pub struct Game {
    shoe: Vec<u8>,
    possible_actions: HashSet<Action>,
    pub player: Player,
    pub dealer: Player,
    pub status: RoundStatus,
}

impl Game {
    pub fn new() -> Game {
        let mut game = Game {
            shoe: Vec::new(),
            player: Player::new(),
            dealer: Player::new(),
            possible_actions: HashSet::new(),
            status: RoundStatus::InProgress,
        };
        game.take_action(Action::Deal);
        game
    }

    pub fn take_action(&mut self, action: Action) {
        match action {
            Action::Deal => {
                if self.shoe.len() < 20 {
                    self.shoe = create_shoe(4);
                }
                self.possible_actions.clear();
                self.dealer.deal_new_hand(&mut self.shoe);
                self.player.deal_new_hand(&mut self.shoe);

                self.status = match (
                    (&mut (self.player)).is_blackjack(),
                    self.dealer.is_blackjack(),
                ) {
                    (true, true) => RoundStatus::Concluded(RoundOutcome::Push),
                    (true, false) => RoundStatus::Concluded(RoundOutcome::Blackjack),
                    (false, true) => RoundStatus::Concluded(RoundOutcome::Lose),
                    (false, false) => RoundStatus::InProgress,
                };
                if self.status == RoundStatus::InProgress {
                    self.possible_actions.insert(Action::Hit);
                    self.possible_actions.insert(Action::Stand);
                } else {
                    self.possible_actions.insert(Action::Deal);
                }
            }
            Action::Hit => {
                self.player.deal_card(&mut self.shoe);
                match self.player.status {
                    HandStatus::Value(_) => (),
                    HandStatus::Bust => self.status = RoundStatus::Concluded(RoundOutcome::Lose),
                }
            }
            Action::Stand => {
                while self.dealer.status < HandStatus::Value(17) {
                    self.dealer.deal_card(&mut self.shoe);
                }
                self.status = match self.dealer.status {
                    HandStatus::Bust => RoundStatus::Concluded(RoundOutcome::Win),
                    HandStatus::Value(d) => {
                        if let HandStatus::Value(p) = self.player.status {
                            match p.cmp(&d) {
                                std::cmp::Ordering::Less => RoundStatus::Concluded(RoundOutcome::Lose),
                                std::cmp::Ordering::Equal => RoundStatus::Concluded(RoundOutcome::Push),
                                std::cmp::Ordering::Greater => RoundStatus::Concluded(RoundOutcome::Win),
                            }
                        } else {
                            RoundStatus::InProgress
                        }
                    }
                }
            }
        };
    }
}

#[derive(Eq, PartialEq, Hash)]
pub enum Action {
    Hit,
    Stand,
    Deal,
}
