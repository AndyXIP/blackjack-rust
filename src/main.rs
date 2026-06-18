use blackjack::Deck;
use blackjack::round::play_round;
use std::io;

fn main() {
    println!("Welcome to Blackjack!");
    println!("How much money do you want to start with?");
    let mut funds_str = String::new();
    io::stdin().read_line(&mut funds_str).unwrap();
    let mut funds: i32 = funds_str.trim().parse().unwrap_or(1);
    let starting_funds = funds;

    while funds > 0 {
        println!("You have £{} available.", funds);
        println!("How much would you like to bet?");
        let mut bet_str = String::new();
        io::stdin().read_line(&mut bet_str).unwrap();
        let bet: i32 = bet_str.trim().parse().unwrap_or(1);

        if bet > funds {
            println!("You cannot bet more than you have!");
            continue;
        }

        if bet <= 0 {
            println!("Please enter a valid bet amount.");
            continue;
        }

        let mut deck = Deck::new();
        deck.shuffle();

        let winnings = play_round(bet, &mut deck);
        funds += winnings;

        println!("You now have £{} available.", funds);

        if funds <= 0 {
            println!("You have run out of money! Game over.");
            break;
        }

        println!("Do you want to play another round? (y/n)");
        let mut response = String::new();
        io::stdin().read_line(&mut response).unwrap();

        if response.trim().to_lowercase() != "y" {
            break;
        }
    }

    println!("Game over! You started with £{} and ended with £{}", starting_funds, funds);
}
