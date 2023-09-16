pub mod shoe;
pub mod hand;
pub mod table;
pub mod action;

pub use shoe::Shoe;
pub use crate::action::{Action, PossibleAction};
use crate::Action::{Double, Hit, Split, Stand, Surrender};
pub use crate::hand::{Hand, HandStatus};
pub use crate::table::{RoundStatus, Table};

pub fn play<'a>(get_action: fn(&Table, Vec<&'a PossibleAction>) -> &'a PossibleAction, get_bet: fn(&Table) -> f64) {
    let mut table = Table {
        shoe: shoe::StandardShoe::new(4),
        player: vec![Hand::new(0.0)],
        dealer: Hand::new(0.0),
        status: RoundStatus::Concluded,
        balance: 0.0,
    };

    loop {
        if table.status == RoundStatus::Concluded {
            table.start_round(get_bet(&table));
        }
        else {
            let mut possible_actions: Vec<&PossibleAction> = Vec::new();
            if table.can_take_basic_actions() {
                possible_actions.push(&PossibleAction(Hit));
                possible_actions.push(&PossibleAction(Stand));
            }
            if table.can_double() {
                possible_actions.push(&PossibleAction(Double));
            }
            if table.can_split() {
                possible_actions.push(&PossibleAction(Split));
            }
            if table.can_surrender() {
                possible_actions.push(&PossibleAction(Surrender));
            }
            table.take_action(get_action(&table, possible_actions).0);
        }
    }
}

pub(crate) fn get_possible_actions(table: &Table) -> Vec<&PossibleAction> {
    let mut possible_actions: Vec<&PossibleAction> = Vec::new();
    if table.can_take_basic_actions() {
        possible_actions.push(&PossibleAction(Hit));
        possible_actions.push(&PossibleAction(Stand));
    }
    if table.can_double() {
        possible_actions.push(&PossibleAction(Double));
    }
    if table.can_split() {
        possible_actions.push(&PossibleAction(Split));
    }
    if table.can_surrender() {
        possible_actions.push(&PossibleAction(Surrender));
    }

    possible_actions
}

