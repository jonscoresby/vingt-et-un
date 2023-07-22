use rand::seq::SliceRandom;
use rand::thread_rng;
use std::io;

fn main() -> io::Result<()> {
    print_banner();
    let mut shoe: Vec<u8> = create_shoe(4);
    loop {
        if shoe.len() < 20 {
            shoe = create_shoe(4)
        };

        match round(&mut shoe) {
            RoundOutcome::Win => println!("You Win"),
            RoundOutcome::Lose => println!("You Lose"),
            RoundOutcome::Push => println!("It's a draw"),
            RoundOutcome::Blackjack => println!("You win with a blackjack"),
        };

        println!("Press 'q' to quit. Press any other key to play again");
        if &get_command() == "q" {
            break;
        }
    }

    Ok(())
}

fn create_shoe(size: u8) -> Vec<u8> {
    println!();
    println!("Shuffling");
    let mut vec = Vec::new();
    (1..size * 4).for_each(|_| {
        vec.extend([1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 10, 10, 10]);
    });
    vec.shuffle(&mut thread_rng());
    vec
}

fn get_command() -> String {
    let stdin = io::stdin();
    let mut user_input = String::new();
    stdin.read_line(&mut user_input).unwrap();
    return user_input.trim().to_owned();
}

fn round(shoe: &mut Vec<u8>) -> RoundOutcome {
    let mut player_hand: Vec<u8> = Vec::new();
    let mut dealer_hand: Vec<u8> = Vec::new();
    player_hand.push(shoe.pop().unwrap());
    dealer_hand.push(shoe.pop().unwrap());
    player_hand.push(shoe.pop().unwrap());
    dealer_hand.push(shoe.pop().unwrap());

    let outcome = match (is_blackjack(&player_hand), is_blackjack(&dealer_hand)) {
        (true, true) => RoundOutcome::Push,
        (true, false) => RoundOutcome::Blackjack,
        (false, true) => RoundOutcome::Lose,
        (false, false) => play(&mut player_hand, &mut dealer_hand, shoe),
    };
    print_hands(&dealer_hand, &player_hand, false);
    outcome
}

fn play( player_hand: &mut Vec<u8>, dealer_hand: &mut Vec<u8>, shoe: &mut Vec<u8>,) -> RoundOutcome {
    match player_turn(player_hand, dealer_hand, shoe) {
        HandStatus::Bust => RoundOutcome::Lose,
        HandStatus::Value(p) => match dealer_turn(dealer_hand, shoe) {
            HandStatus::Bust => RoundOutcome::Win,
            HandStatus::Value(d) => match p.cmp(&d) {
                std::cmp::Ordering::Less => RoundOutcome::Lose,
                std::cmp::Ordering::Equal => RoundOutcome::Push,
                std::cmp::Ordering::Greater => RoundOutcome::Win,
            },
        },
    }
}

fn dealer_turn(dealer_hand: &mut Vec<u8>, shoe: &mut Vec<u8>) -> HandStatus {
    let mut hand_status = get_hand_status(&dealer_hand);
    while match hand_status {
        HandStatus::Value(n) if n < 17 => true,
        _ => false,
    } {
        dealer_hand.push(shoe.pop().unwrap());
        hand_status = get_hand_status(&dealer_hand);
    }
    hand_status
}

fn player_turn(player_hand: &mut Vec<u8>, dealer_hand: &Vec<u8>, shoe: &mut Vec<u8>) -> HandStatus {
    'l: loop {
        print_hands(dealer_hand, player_hand, true);
        println!("Choose an action: (h)it, (s)tand ");
        loop {
            let mut hand_status = get_hand_status(player_hand);
            match &get_command() as &str {
                "h" => {
                    player_hand.push(shoe.pop().unwrap());
                    hand_status = get_hand_status(player_hand);
                    match hand_status {
                        HandStatus::Value(n) if n < 21 => break,
                        _ => break 'l hand_status,
                    }
                }
                "s" => break 'l hand_status,
                _ => println!("That is not valid action! Try again."),
            }
        }
    }
}

fn get_hand_status(hand: &Vec<u8>) -> HandStatus {
    let mut aces: u8 = hand.iter().filter(|&n| *n == 1).count().try_into().unwrap();
    let mut base_sum = aces * 10 + hand.iter().sum::<u8>();

    while base_sum > 21 && aces > 0 {
        base_sum -= 10;
        aces -= 1;
    }
    if base_sum > 21 {
        HandStatus::Bust
    } else {
        HandStatus::Value(base_sum)
    }
}

#[derive(PartialEq)]
enum HandStatus {
    Bust,
    Value(u8),
}

impl HandStatus {
    fn is_blackjack(self) -> bool {
        self == HandStatus::Value(21)
    }
}

fn is_blackjack(hand: &Vec<u8>) -> bool {
    get_hand_status(hand).is_blackjack()
}

enum RoundOutcome {
    Win,
    Lose,
    Push,
    Blackjack,
}

fn print_hands(dealer_hand: &Vec<u8>, player_hand: &Vec<u8>, hide_hole_card: bool) {
    println!();

    print!("Dealer Hand: ");
    if hide_hole_card {
        println!("[?, {}]", dealer_hand[1])
    } else {
        println!("{:?}", dealer_hand);
    }

    println!("Player Hand: {:?}", player_hand);
    println!();
}

#[rustfmt::skip]
pub fn print_banner() {
    println!( r"   /$$$$$$$  /$$                     /$$          /$$$$$                     /$$      " );
    println!( r"  | $$__  $$| $$                    | $$         |__  $$                    | $$      " );
    println!( r"  | $$  \ $$| $$  /$$$$$$   /$$$$$$$| $$   /$$      | $$  /$$$$$$   /$$$$$$$| $$   /$$" );
    println!( r"  | $$$$$$$ | $$ |____  $$ /$$_____/| $$  /$$/      | $$ |____  $$ /$$_____/| $$  /$$/" );
    println!( r"  | $$__  $$| $$  /$$$$$$$| $$      | $$$$$$/  /$$  | $$  /$$$$$$$| $$      | $$$$$$/ " );
    println!( r"  | $$  \ $$| $$ /$$__  $$| $$      | $$_  $$ | $$  | $$ /$$__  $$| $$      | $$_  $$ " );
    println!( r"  | $$$$$$$/| $$|  $$$$$$$|  $$$$$$$| $$ \  $$|  $$$$$$/|  $$$$$$$|  $$$$$$$| $$ \  $$" );
    println!( r"  |_______/ |__/ \_______/ \_______/|__/  \__/ \______/  \_______/ \_______/|__/  \__/" );
    println!();
    println!();
}
