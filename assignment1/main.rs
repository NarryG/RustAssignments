use rand::seq::{IndexedRandom};
use std::io;
#[derive(Eq, PartialEq, Clone, Copy)]

enum Symbol {
    Blank,
    Blood,
    Bat,
    Vampire,
}

enum UserChoice {
    Bet,
    Stop,
}

impl Symbol {
    fn to_emoji(self) -> &'static str {
        match self {
            Symbol::Blank => "🦉", // Blank creature 
            Symbol::Bat => "🦇",
            Symbol::Blood => "🩸",
            Symbol::Vampire => "🧛",
        }
    }
}

fn print_stake_and_win() {
    println!("Stake & Win");
}

fn print_symbols(s0: Symbol, s1: Symbol, s2: Symbol) {
    println!("------------------");
    println!(
        "|| {} | {} | {} ||",
        s0.to_emoji(),
        s1.to_emoji(),
        s2.to_emoji(),
    );
    println!("------------------");
}

fn get_choice() -> UserChoice {
    loop {
        let mut input: String = String::new();
        println!("{{S}} -> Stop | Enter -> Spin!");

        io::stdin()
            .read_line(&mut input)
            .expect("Unable to read line");

        match input.to_lowercase().trim() {
            "s" =>  return UserChoice::Stop,
            s if s.is_empty() =>  return UserChoice::Bet,
            _ => "",
        };
    }
}

fn spin_reel() -> Symbol {
    let reel = vec![Symbol::Bat, Symbol::Blank, Symbol::Blood, Symbol::Vampire];
    let mut rng = rand::rng();
    let choice = reel.choose(&mut rng).expect("sorry nothing");

    return *choice;
}

fn score_outcome(s0: Symbol, s1: Symbol, s2: Symbol) -> (i64, Option<&'static str>) {
    match (s0, s1, s2) {
        (Symbol::Blood, Symbol::Blood, Symbol::Blood) => (5, Some("Three blood!")),
        (Symbol::Bat, Symbol::Bat, Symbol::Bat) => (10, Some("Three bat!")),
        (Symbol::Vampire, Symbol::Vampire, Symbol::Vampire) => (15, Some("Three vampires!")),
        _ => (0, None),
    }
}

fn game_loop() {
    let mut coins: i64 = 100;
    let mut s0 = Symbol::Blank;
    let mut s1 = Symbol::Blank;
    let mut s2 = Symbol::Blank;

    print_stake_and_win();
    print_symbols(s0, s1, s2);
    
    loop {
        println!("Coins: {coins}");
        if coins <= 0 {
            println!("You're poor and we don't accept blood (yet). Goodbye!");
            return;
        }

        match get_choice() {
            UserChoice::Bet => {
                coins -= 1;

                s0 = spin_reel();
                s1 = spin_reel();
                s2 = spin_reel();
                print_symbols(s0, s1, s2);
                let (score, msg) = score_outcome(s0, s1, s2);
                coins += score;
                if let Some(text) = msg {
                    println!("{text}");
                }
            }
            UserChoice::Stop => {
                println!("See you soon!");
                break;
            }
        }
    }
}

fn main() {
    game_loop();
}
