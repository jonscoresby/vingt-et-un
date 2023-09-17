use crate::action::Action;
use crate::hand::{Hand, HandStatus};
use crate::shoe::Shoe;
use crate::Action::{Double, Hit, Split, Stand, Surrender};
use crate::PossibleAction;

#[derive(PartialEq, Debug)]
pub enum RoundStatus {
    Concluded,
    InProgress(usize),
}

pub struct Table {
    pub(crate) shoe: Box<dyn Shoe>,
    pub player: Vec<Hand>,
    pub dealer: Hand,
    pub status: RoundStatus,
    pub balance: f64,
}

impl Table {
    pub(crate) fn take_action(&mut self, action: Action) {
        if let RoundStatus::InProgress(active_hand_index) = self.status {
            match action {
                Stand => self.next_hand(),
                Hit => {
                    if self.player[active_hand_index].deal_card(&mut self.shoe) >= 21 {
                        self.next_hand();
                    }
                }
                Double => {
                    self.balance -= self.player[active_hand_index].bet_amount;
                    self.player[active_hand_index].bet_amount *= 2.0;
                    self.take_action(Hit)
                }
                Split => {
                    self.balance -= self.player[active_hand_index].bet_amount;
                    let (new_hand, old_hand_value) =
                        self.player[active_hand_index].split(&mut self.shoe);
                    self.player.insert(active_hand_index + 1, new_hand);
                    if old_hand_value >= 21 {
                        self.next_hand()
                    }
                }
                Surrender => {
                    self.balance += self.player[active_hand_index].bet_amount / 2.0;
                    self.player[active_hand_index].status = HandStatus::Surrender;
                    self.next_hand();
                }
            }
        }
    }

    pub(crate) fn start_round(&mut self, bet_amount: f64) {
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

    pub(crate) fn get_possible_actions(&self) -> Vec<PossibleAction> {
        let mut possible_actions: Vec<PossibleAction> = Vec::new();
        if self.can_take_basic_actions() {
            possible_actions.push(PossibleAction(Hit));
            possible_actions.push(PossibleAction(Stand));
        }
        if self.can_double() {
            possible_actions.push(PossibleAction(Double));
        }
        if self.can_split() {
            possible_actions.push(PossibleAction(Split));
        }
        if self.can_surrender() {
            possible_actions.push(PossibleAction(Surrender));
        }

        possible_actions
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
