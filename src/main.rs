use std::io;
use rand::Rng;
use std::collections::HashMap;

fn main() {
    println!("Rock, Paper, Scissors");
    let mut user_score = 0;
    let mut computer_score = 0;
    let mut ties_counter = 0;

    loop {
        println!("Please input your object. Or type 'exit' to quit.");

        let mut user_object = String::new();
        let mut computer_object = String::new();

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
        } else if user_object == "exit" {
            break;
        } else {
            continue;
        }
    }
}

fn choseRandomGameItem() -> String {
    // TODO: need some kind of map where the key can be an int
    // let objects_map: HashMap<i32, String> = [(0, "rock"), (1, "paper"), (2, "scissors")].iter().cloned().collect();
    let computer_choice_number = rand::thread_rng().gen_range(0, 3);
    // println!("Computer chooses: {}", computer_choice_number);
    "".into()
}

fn whoWins (user_object: String, computer_object: String) -> String {
    "test".into()
}
