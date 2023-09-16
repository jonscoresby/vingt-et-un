use console::Term;
use rand::seq::SliceRandom;
use std::collections::HashMap;
use vingt_et_un::*;

struct StandardShoe {
    deck: Vec<u8>,
    size: u8,
}
impl StandardShoe {
    fn new(size: u8) -> StandardShoe {
        let mut vec = Vec::new();
        (0..size * 4).for_each(|_| {
            vec.extend([1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 10, 10, 10]);
        });
        vec.shuffle(&mut rand::thread_rng());

        StandardShoe { deck: vec, size }
    }
}
impl Shoe for StandardShoe {
    fn deal(&mut self) -> u8 {
        self.deck.pop().unwrap()
    }

    fn on_new_round(&mut self) {
        if self.deck.len() < 20 {
            *self = StandardShoe::new(self.size);
        }
    }
}

fn main() {
    print_banner();
    let term = Term::stdout();

    let mut game = Table::new(Box::new(StandardShoe::new(4)));
    let mut possible_actions = HashMap::<char, Action>::new();

    loop {
        if !game.player[0].cards.is_empty() {
            print_game(
                &term,
                &game.dealer,
                &game.player,
                &game.status,
                game.balance,
            );
        }

        possible_actions.clear();
        let mut prompt = "Choose an action: ".to_owned();
        if game.can_deal() {
            let bet = game.player[0].bet_amount;
            if bet > 0.0 {
                prompt += "(r)ebet, ";
                possible_actions.insert('r', Action::Deal(bet));
            }
            prompt += "(n)ew bet, ";
        }
        if game.can_take_basic_actions() {
            possible_actions.insert('h', Action::Hit);
            possible_actions.insert('s', Action::Stand);
            prompt += "(h)it, (s)tand, "
        }
        if game.can_double() {
            possible_actions.insert('d', Action::Double);
            prompt += "(d)ouble, ";
        }
        if game.can_split() {
            possible_actions.insert('l', Action::Split);
            prompt += "sp(l)it, ";
        }
        if game.can_surrender() {
            possible_actions.insert('u', Action::Surrender);
            prompt += "s(u)rrender, ";
        }
        prompt += "(q)uit";
        println!("{}", prompt);

        let command = term.read_char().unwrap();
        match match command {
            'q' => break,
            'n' => game.take_action(Action::Deal(get_bet_amount(&term))),
            _ => match possible_actions.get(&command) {
                Some(action) => game.take_action(*action),
                None => Err(()),
            },
        } {
            Err(_) => print!("Not a valid action! Try again."),
            _ => (),
        }
    }
}

fn get_bet_amount(term: &Term) -> f64 {
    println!("Enter a new bet amount:");
    loop {
        let new_bet: Result<f64, _> = term.read_line().unwrap().parse();
        if new_bet.is_ok() {
            return new_bet.unwrap();
        }
        println!("Invalid value. Try again.")
    }
}

fn print_game(
    term: &Term,
    dealer: &Hand,
    player: &Vec<Hand>,
    active_hand: &RoundStatus,
    balance: f64,
) {
    term.clear_screen().unwrap();
    println!();
    print!(" Dealer Hand: ");
    if let RoundStatus::InProgress(n) = active_hand {
        println!("[?, {}]", dealer.cards[1]);
        player.iter().enumerate().for_each(|(i, hand)| {
            println!(
                "{}Player Hand: {:?} {} {}   Bet: ${}",
                if *n == i { ">" } else { " " },
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
            dealer.cards,
            dealer.value,
            hand_message(dealer.status)
        );
        player.iter().for_each(|hand| {
            println!(
                " Player Hand: {:?} {} {}  Bet: ${} ",
                hand.cards,
                hand.value,
                hand_message(hand.status),
                hand.bet_amount,
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
