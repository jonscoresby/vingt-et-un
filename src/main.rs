use console::Term;
use std::collections::HashMap;
use vingt_et_un::shoe::StandardShoe;
use vingt_et_un::{Action, Game, HandStatus, PlayerBalanceError, PossibleAction, Round};

fn main() {
    print_banner();
    Game::start_game(StandardShoe::new(4), new_round, get_action);
}

fn new_round(game: &mut Game, last_round: &Round) {
    if game.get_player_balances().is_empty() {
        // update_player_balance returns an error if balance is negative. this
        // hardcoded value is positive, so unwrap is safe
        game.set_player_balances(vec![1000.0]).unwrap();
    } else {
        print_game(last_round)
    }

    println!("Enter a new bet amount:");
    loop {
        match Term::stdout()
            .read_line_initial_text(&game.get_bet(0, 0).unwrap().to_string())
            .unwrap()
            .parse()
        {
            Ok(x) => match game.set_bet(0, 0, x) {
                Ok(_) => break,
                Err(x) if matches!(x, PlayerBalanceError::BalanceCannotBeNegative) => {
                    println!("Your balance is too low to bet that much. Enter lower bet.")
                }
                _ => panic!(), // player index and hand index of zero should always work
            },
            Err(_) => println!("That wasn't a valid number. Try again."),
        }
    }
}

fn get_action(game: &Round, possible: Vec<PossibleAction>) -> PossibleAction {
    print_game(game);

    let mut prompt = "Choose an action: ".to_owned();
    let mut possible_actions = HashMap::<char, PossibleAction>::new();
    for i in possible {
        match i.action() {
            Action::Hit => {
                possible_actions.insert('h', i);
                prompt += "(h)it, "
            }
            Action::Stand => {
                possible_actions.insert('s', i);
                prompt += "(s)tand, "
            }
            Action::Double => {
                possible_actions.insert('d', i);
                prompt += "(d)ouble, "
            }
            Action::Split => {
                possible_actions.insert('l', i);
                prompt += "sp(l)it, ";
            }
            Action::Surrender => {
                possible_actions.insert('u', i);
                prompt += "s(u)rrender, "
            }
        }
    }

    prompt += "(q)uit";
    println!("{}", prompt);

    loop {
        match Term::stdout().read_char().unwrap() {
            'q' => std::process::exit(0),
            c => match possible_actions.remove(&c) {
                None => println!("Invalid action"),
                Some(x) => break x,
            },
        }
    }
}

fn print_game(round: &Round) {
    Term::stdout().clear_screen().unwrap();
    println!();
    print!(" Dealer Hand: ");
    if round.active_hand_index != round.player_hands.len() {
        println!("[?, {}]", round.dealer.cards[1]);
        round
            .player_hands
            .iter()
            .enumerate()
            .for_each(|(i, position)| {
                println!(
                    "{}Player Hand: {:?} {} {}   Bet: ${}",
                    if round.active_hand_index == i {
                        ">"
                    } else {
                        " "
                    },
                    position.hand.cards,
                    if position.hand.soft {
                        format!("{}/{}", position.hand.value - 10, position.hand.value)
                    } else {
                        position.hand.value.to_string()
                    },
                    hand_message(position.hand.status),
                    position.bet_amount,
                )
            });
    } else {
        println!(
            "{:?} {} {}",
            round.dealer.cards,
            round.dealer.value,
            hand_message(round.dealer.status)
        );
        round.player_hands.iter().for_each(|position| {
            println!(
                " Player Hand: {:?} {} {}  Bet: ${} ",
                position.hand.cards,
                position.hand.value,
                hand_message(position.hand.status),
                position.bet_amount,
            )
        });
    }

    println!("Current Balance: {}", round.player_hands[0].balance());
    println!();
}

fn hand_message(status: HandStatus) -> &'static str {
    match status {
        HandStatus::Win => "Win",
        HandStatus::Lose => "Lost",
        HandStatus::Push => "Push",
        HandStatus::Blackjack => "Blackjack",
        HandStatus::Bust => "Bust",
        HandStatus::Surrender => "Surrender",
        _ => "",
    }
}

#[rustfmt::skip]
pub fn print_banner() {
    print!("");
    println!(r"   /$$    /$$ /$$                       /$$           /$$$$$$$$ /$$           /$$   /$$          ");
    println!(r"  | $$   | $$|__/                      | $$          | $$_____/| $$          | $$  | $$          ");
    println!(r"  | $$   | $$ /$$ /$$$$$$$   /$$$$$$  /$$$$$$        | $$     /$$$$$$        | $$  | $$ /$$$$$$$ ");
    println!(r"  |  $$ / $$/| $$| $$__  $$ /$$__  $$|_  $$_/        | $$$$$ |_  $$_/        | $$  | $$| $$__  $$");
    println!(r"   \  $$ $$/ | $$| $$  \ $$| $$  \ $$  | $$          | $$__/   | $$          | $$  | $$| $$  \ $$");
    println!(r"    \  $$$/  | $$| $$  | $$| $$  | $$  | $$ /$$      | $$      | $$ /$$      | $$  | $$| $$  | $$");
    println!(r"     \  $/   | $$| $$  | $$|  $$$$$$$  |  $$$$/      | $$$$$$$$|  $$$$/      |  $$$$$$/| $$  | $$");
    println!(r"      \_/    |__/|__/  |__/ \____  $$   \___/        |________/ \___/         \______/ |__/  |__/");
    println!(r"                            /$$  \ $$                                                            ");
    println!(r"                           |  $$$$$$/                                                            ");
    println!(r"                            \______/                                                             ");
}
