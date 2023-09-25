use console::Term;
use std::collections::HashMap;
use vingt_et_un::{play, Action, HandStatus, PossibleAction, RoundStatus, Table};
use vingt_et_un::table::Player;

fn main() {
    print_banner();
    let player = Player::new();
    play(get_action, get_bet);
}

fn get_action(game: &Table, possible: Vec<PossibleAction>) -> PossibleAction {
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

fn get_bet(game: &Table) -> f64 {
    print_game(game);
    println!("Choose an action: (r)ebet, (n)ew bet, (q)uit");
    loop {
        match Term::stdout().read_char().unwrap() {
            'q' => std::process::exit(0),
            'r' => break game.positions[0].bet_amount,
            'n' => break get_bet_amount(),
            _ => println!("Invalid action"),
        }
    }
}

fn get_bet_amount() -> f64 {
    println!("Enter a new bet amount:");
    loop {
        let new_bet: Result<f64, _> = Term::stdout().read_line().unwrap().parse();
        match new_bet {
            Ok(x) => break x,
            Err(_) => println!("Invalid value. Try again."),
        }
    }
}

fn print_game(game: &Table) {
    Term::stdout().clear_screen().unwrap();
    println!();
    print!(" Dealer Hand: ");
    if let RoundStatus::InProgress(n) = game.status {
        println!("[?, {}]", game.dealer.cards[1]);
        game.positions.iter().enumerate().for_each(|(i, hand)| {
            println!(
                "{}Player Hand: {:?} {} {}   Bet: ${}",
                if n == i { ">" } else { " " },
                hand.cards,
                if hand.soft {
                    format!("{}/{}", hand.value - 10, hand.value)
                } else {
                    hand.value.to_string()
                },
                hand_message(hand.status),
                hand.bet_amount,
            )
        });
    } else {
        println!(
            "{:?} {} {}",
            game.dealer.cards,
            game.dealer.value,
            hand_message(game.dealer.status)
        );
        game.positions.iter().for_each(|hand| {
            println!(
                " Player Hand: {:?} {} {}  Bet: ${} ",
                hand.cards,
                hand.value,
                hand_message(hand.status),
                hand.bet_amount,
            )
        });
    }

    println!("Current Balance: {}", 0);
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
        HandStatus::Value => "",
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
