pub mod action;
pub mod hand;
pub mod shoe;
pub mod table;

pub use crate::action::{Action, PossibleAction};
use crate::hand::Player;
pub use crate::hand::{Hand, HandStatus};
use crate::shoe::Shoe;
use shoe::StandardShoe;

// pub fn play(
//     get_action: fn(&Table, Vec<PossibleAction>) -> PossibleAction,
//     get_bet: fn(Option<&Table>) -> f64,
// ) {
//     let y = Player{ balance: 0.0 };
//     let mut shoe: Box<dyn Shoe> = StandardShoe::new(4);
//     let mut table = Table::calculate_round_start(&mut shoe, vec![Hand::new(get_bet(None), &y)], );
//     loop {
//         if let RoundStatus::InProgress(active_hand_index) = table.status {
//             let other_possible = table.get_possible_actions();
//             table.take_action(active_hand_index, get_action(&table, other_possible).0);
//         } else {
//             table = Table::calculate_round_start(&mut shoe, vec![Hand::new(get_bet(Option::from(&table)), &y)], get_action);
//         }
//     }
// }
