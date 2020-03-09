use std::io;
use rand::Rng;
// use std::collections::HashMap;
use intmap::IntMap;

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
            println!("Computer chooses {}", chose_random_game_item());
        } else if user_object == "paper" {
            println!("Computer chooses {}", chose_random_game_item());
        } else if user_object == "scissors" {
            println!("Computer chooses {}", chose_random_game_item());
        } else if user_object == "exit" {
            break;
        } else {
            continue;
        }
    }
}

fn chose_random_game_item() -> String {
    let mut objects_map = IntMap::new();
    let computer_choice_number = rand::thread_rng().gen_range(0, 3);

    objects_map.insert(0, "rock");
    objects_map.insert(1, "paper");
    objects_map.insert(2, "scissors");
    objects_map.get(computer_choice_number).unwrap().to_string()
}

fn whoWins (user_object: String, computer_object: String) -> String {
    "test".into()
}
