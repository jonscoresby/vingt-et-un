use std::{collections::HashMap, io};

use vingt_et_un::{Action, Hand, HandStatus, RoundStatus, Table};

fn main() -> io::Result<()> {
    print_banner();
    let mut game = Table::new(100.0);
    let mut possible_actions = HashMap::<&str, Action>::new();

    loop {
        print_game(&game.dealer, &game.player, &game.status, game.balance);

        possible_actions.clear();
        let mut prompt = "Choose an action: ".to_owned();
        if game.can_deal() {
            let bet = game.player[0].bet_amount;
            if bet > 0.0 {
                prompt += "(r)ebet, ";
                possible_actions.insert("r", Action::Deal(bet));
            }
            prompt += "(n)ew bet, ";
        }
        if game.can_take_basic_actions() {
            possible_actions.insert("h", Action::Hit);
            possible_actions.insert("s", Action::Stand);
            prompt += "(h)it, (s)tand, "
        }
        if game.can_double() {
            possible_actions.insert("d", Action::Double);
            prompt += "(d)ouble, ";
        }
        if game.can_split() {
            possible_actions.insert("l", Action::Split);
            prompt += "sp(l)it, ";
        }
        if game.can_surrender() {
            possible_actions.insert("u", Action::Surrender);
            prompt += "s(u)rrender, ";
        }
        prompt += "(q)uit";
        println!("{}", prompt);

        let command = get_command();
        match command.as_str() {
            "q" => break,
            "n" => game.take_action(Action::Deal(get_bet_amount())),
            _ => match possible_actions.get(command.as_str()) {
                Some(action) => game.take_action(*action),
                None => println!("That is not valid action! Try again."),
            },
        }
    }

    Ok(())
}

fn get_command() -> String {
    let stdin = io::stdin();
    let mut user_input = String::new();
    stdin.read_line(&mut user_input).unwrap();
    return user_input.trim().to_owned();
}

fn get_bet_amount() -> f64 {
    loop {
        println!("Enter a new bet amount:");
        let new_bet: Result<f64, _> = get_command().parse();
        if new_bet.is_ok() {
            return new_bet.unwrap();
        }
        print!("Invalid value. Try again.")
    }
}

fn print_game(dealer: &Hand, player: &Vec<Hand>, active_hand: &RoundStatus, balance: f64) {
    println!();
    print!(" Dealer Hand: ");
    if let RoundStatus::InProgress(n) = active_hand {
        println!("[?, {}]", dealer.cards[1]);
        player.iter().enumerate().for_each(|(i, hand)| {
            println!(
                "{}Player Hand: {:?} {} {} {}",
                if *n == i { ">" } else { " " },
                hand.cards,
                if hand.soft {
                    format!("{}/{}", hand.value - 10, hand.value)
                } else {
                    hand.value.to_string()
                },
                hand.bet_amount,
                hand_message(hand.status)
            )
        });
    } else {
        println!(
            "{:?} {} {}",
            dealer.cards,
            dealer.value,
            hand_message(dealer.status)
        );
        player.iter().for_each(|hand| {
            println!(
                " Player Hand: {:?} {} {} {}",
                hand.cards,
                hand.value,
                hand.bet_amount,
                hand_message(hand.status)
            )
        });
    }

    println!("Current Balance: {}", balance);
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
