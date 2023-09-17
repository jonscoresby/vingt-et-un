pub mod action;
pub mod hand;
pub mod shoe;
pub mod table;

pub use crate::action::{Action, PossibleAction};
pub use crate::hand::{Hand, HandStatus};
pub use crate::table::{RoundStatus, Table};
use shoe::StandardShoe;

pub fn play(
    get_action: fn(&Table, Vec<PossibleAction>) -> PossibleAction,
    get_bet: fn(&Table) -> f64,
) {
    let mut table = Table {
        shoe: StandardShoe::new(4),
        player: vec![Hand::new(0.0)],
        dealer: Hand::new(0.0),
        status: RoundStatus::Concluded,
        balance: 0.0,
    };

    loop {
        if table.status == RoundStatus::Concluded {
            table.start_round(get_bet(&table));
        } else {
            let other_possible = table.get_possible_actions();
            table.take_action(get_action(&table, other_possible).0);
        }
    }
}
