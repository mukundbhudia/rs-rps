use std::io;
use rand::Rng;

fn main() {
    println!("Rock, Paper, Scissors");

    loop {
        let computer_choice_number = rand::thread_rng().gen_range(0, 3);
        println!("Computer chooses: {}", computer_choice_number);

        println!("Please input your object.");

        let mut user_object = String::new();

        io::stdin().read_line(&mut user_object)
            .expect("Failed to read line");

        let user_object = user_object.trim();

        println!("You entered: {}", user_object);

        if user_object == "rock" {
            println!("ROCK!");
        } else if user_object == "paper" {
            println!("PAPER!");
        } else if user_object == "scissors" {
            println!("SCISSORS!");
        } else {
            continue;
        }
    }
}
