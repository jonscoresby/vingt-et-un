use crate::action::Action;
use crate::hand::{Hand, HandStatus, Player, Position};
use crate::shoe::Shoe;
use crate::Action::{Double, Hit, Split, Stand, Surrender};
use crate::PossibleAction;
use crate::RoundStatus::InProgress;

#[derive(PartialEq, Debug)]
pub enum RoundStatus {
    Concluded,
    InProgress(usize),
}
pub struct Table<'a> {
    pub(crate) shoe: &'a mut Box<dyn Shoe>,
    pub positions: Vec<Position<'a>>,
    pub dealer: Hand,
    pub status: RoundStatus,
}

impl<'a> Table<'a> {
    pub(crate) fn take_action(active_position_index: usize, action: Action) {
        let active_position = &mut self.positions[active_position_index];
            match action {
                Stand => self.status = InProgress(active_position_index + 1),
                Hit => {active_position.hand.deal_card(&mut self.shoe);}
                Double => {
                    active_position.player.borrow_mut().add_amount(-active_position.bet_amount);
                    active_position.bet_amount *= 2.0;
                    active_position.hand.deal_card(&mut self.shoe);
                }
                Split => {
                    active_position.player.borrow_mut().add_amount(-active_position.bet_amount);
                    self.positions.insert(active_position_index + 1, active_position.split(&mut self.shoe));
                }
                Surrender => {
                    active_position.player.borrow_mut().add_amount(active_position.bet_amount / 2.0);
                    active_position.hand.status = HandStatus::Surrender;
                }
            }}

    // this can be optimized
    pub(crate) fn calculate_round_start(dealer: &Hand, positions: &mut Vec<Position>) {
        if let HandStatus::Blackjack = dealer.status{
            for mut position in positions {
                if let HandStatus::Blackjack = position.hand.status{
                    position.player.borrow_mut().add_amount(position.bet_amount);
                    position.hand.status =HandStatus::Push;
                }else {
                    position.hand.status = HandStatus::Lose;
                }
            }
        } else {
            for hand in positions {
                if let HandStatus::Blackjack = hand.hand.status{
                    hand.player.borrow_mut().add_amount(hand.bet_amount * 5.0 / 2.0);
                }
            }
        }
    }

    fn play_round(shoe: &mut Box<dyn Shoe>, mut positions: Vec<Position>, get_action: fn(&Table, Vec<PossibleAction>) -> PossibleAction){
        let dealer = Hand::new(shoe);
        Self::calculate_round_start(&dealer, &mut positions);
        let status = Self::update_round_status(&positions, 0);

        loop {
            if let InProgress(x) = status {
                Self::take_action(x, get_action(&round, round.get_possible_actions()).action());
                Self::update_round_status(&round.positions, x);
            } else {
                if positions.iter().all(|position| position.hand.status == HandStatus::Bust || position.hand.status == HandStatus::Surrender) { break; }
                round.dealer_turn();
                break
            }
        }
    }

    fn update_round_status(positions: &Vec<Position>, i: usize) -> RoundStatus {
        if i == positions.len() {
            RoundStatus::Concluded
        } else if positions[i].hand.value == 21 {
            Self::update_round_status(positions, i + 1)
        } else {
            InProgress(i)
        }
    }

    fn calculate_round_end(dealer: &Hand, positions: &mut Vec<Position>){
        for posistion in positions {
            if let HandStatus::Value = posistion.hand.status {
                posistion.hand.status = if let HandStatus::Value = dealer.status {
                    match posistion.hand.value.cmp(&dealer.value) {
                        std::cmp::Ordering::Less => HandStatus::Lose,
                        std::cmp::Ordering::Equal => HandStatus::Push,
                        std::cmp::Ordering::Greater => HandStatus::Win,
                    }
                } else {
                    HandStatus::Win
                }
            }
            posistion.player.borrow_mut().add_amount(match posistion.hand.status {
                HandStatus::Win => posistion.bet_amount * 2.0,
                HandStatus::Push => posistion.bet_amount,
                _ => 0.0,
            });
        }
    }

    // move this to position and add bet amount validation
    pub(crate) fn get_possible_actions(&self) -> Vec<PossibleAction> {
        let mut possible_actions: Vec<PossibleAction> = Vec::new();

        possible_actions.push(PossibleAction(Hit));
        possible_actions.push(PossibleAction(Stand));
        possible_actions.push(PossibleAction(Double));

        if let InProgress(i) = self.status {
            if self.positions[i].hand.can_split() {
                possible_actions.push(PossibleAction(Split));
            }
            if self.positions.len() == 1 && self.positions[i].hand.cards.len() == 2 {
                possible_actions.push(PossibleAction(Surrender));
            }
        }

        possible_actions
    }
}
