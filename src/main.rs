use rand::seq::SliceRandom;
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
    vec.shuffle(&mut rand::thread_rng());
    vec
}

fn get_command() -> String {
    let stdin = io::stdin();
    let mut user_input = String::new();
    stdin.read_line(&mut user_input).unwrap();
    return user_input.trim().to_owned();
}

fn round(shoe: &mut Vec<u8>) -> RoundOutcome {
    let mut player = Player::new();
    let mut dealer = Player::new();
    player.add_card(shoe.pop().unwrap());
    dealer.add_card(shoe.pop().unwrap());
    player.add_card(shoe.pop().unwrap());
    dealer.add_card(shoe.pop().unwrap());

    let outcome = match (player.is_blackjack(), dealer.is_blackjack()) {
        (true, true) => RoundOutcome::Push,
        (true, false) => RoundOutcome::Blackjack,
        (false, true) => RoundOutcome::Lose,
        (false, false) => play(&mut player, &mut dealer, shoe),
    };
    print_hands(&dealer, &player, false);
    outcome
}

fn play(player: &mut Player, dealer: &mut Player, shoe: &mut Vec<u8>) -> RoundOutcome {
    player_turn(player, dealer, shoe);
    match player.status {
        HandStatus::Bust => RoundOutcome::Lose,
        HandStatus::Value(p) => {
            dealer_turn(dealer, shoe);
            match dealer.status {
                HandStatus::Bust => RoundOutcome::Win,
                HandStatus::Value(d) => match p.cmp(&d) {
                    std::cmp::Ordering::Less => RoundOutcome::Lose,
                    std::cmp::Ordering::Equal => RoundOutcome::Push,
                    std::cmp::Ordering::Greater => RoundOutcome::Win,
                },
            }
        }
    }
}

fn dealer_turn(dealer: &mut Player, shoe: &mut Vec<u8>) {
    while dealer.status < HandStatus::Value(17) {
        dealer.add_card(shoe.pop().unwrap());
    }
}

fn player_turn(player: &mut Player, dealer: &Player, shoe: &mut Vec<u8>) {
    'l: loop {
        print_hands(dealer, player, true);
        println!("Choose an action: (h)it, (s)tand ");
        loop {
            match &get_command() as &str {
                "h" => {
                    if *player.add_card(shoe.pop().unwrap()) == HandStatus::Bust {
                        break 'l;
                    } else {
                        break;
                    }
                }
                "s" => break 'l,
                _ => println!("That is not valid action! Try again."),
            }
        }
    }
}

struct Player {
    hand: Vec<u8>,
    status: HandStatus,
}

impl Player {
    fn new() -> Player {
        Player {
            hand: Vec::new(),
            status: HandStatus::Value(0),
        }
    }

    fn add_card(&mut self, card: u8) -> &HandStatus {
        self.hand.push(card);

        let mut aces: u8 = self
            .hand
            .iter()
            .filter(|&n| *n == 1)
            .count()
            .try_into()
            .unwrap();
        let mut base_sum = aces * 10 + self.hand.iter().sum::<u8>();

        while base_sum > 21 && aces > 0 {
            base_sum -= 10;
            aces -= 1;
        }
        self.status = if base_sum > 21 {
            HandStatus::Bust
        } else {
            HandStatus::Value(base_sum)
        };
        &self.status
    }

    fn is_blackjack(&self) -> bool {
        self.status == HandStatus::Value(21)
    }
}

#[derive(PartialEq, PartialOrd)]
enum HandStatus {
    Value(u8),
    Bust,
}

enum RoundOutcome {
    Win,
    Lose,
    Push,
    Blackjack,
}

fn print_hands(dealer: &Player, player: &Player, hide_hole_card: bool) {
    println!();

    print!("Dealer Hand: ");
    if hide_hole_card {
        println!("[?, {}]", dealer.hand[1])
    } else {
        println!("{:?}", dealer.hand);
    }

    println!("Player Hand: {:?}", player.hand);
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
