use console::Term;
use std::collections::HashMap;
use vingt_et_un::shoe::StandardShoe;
use vingt_et_un::table::{Game, };
use vingt_et_un::{Action, HandStatus, PossibleAction};
use vingt_et_un::round::Round;

fn main() {
    print_banner();
    Game::start_game(StandardShoe::new(4), new_round, get_action);
}

fn new_round(game: &mut Game, last_round: &Round){
    if game.get_player_balances().is_empty() {
        // update_player_balance returns an error if balance is negative. this
        // hardcoded value is positive, so unwrap is safe
        game.update_player_balances(vec![100.0]).unwrap();
    }else {
        print_game(last_round)
    }

    println!("Choose an action: (r)ebet, (n)ew bet, (q)uit");
    let bet_amount = loop {
        match Term::stdout().read_char().unwrap() {
            'q' => std::process::exit(0),
            'r' => break last_round.player_hands[0].bet_amount,
            'n' => break get_bet_amount(),
            _ => println!("Invalid action"),
        }
    };

    game.set_bet(0, bet_amount);
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

fn print_game(round: &Round) {
    Term::stdout().clear_screen().unwrap();
    println!();
    print!(" Dealer Hand: ");
    if round.active_hand_index != round.player_hands.len() {
        println!("[?, {}]", round.dealer.cards[1]);
        round.player_hands.iter().enumerate().for_each(|(i, position)| {
            println!(
                "{}Player Hand: {:?} {} {}   Bet: ${}",
                if round.active_hand_index == i { ">" } else { " " },
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
