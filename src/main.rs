use std::io;
use rand::Rng;
use std::collections::HashMap;
use intmap::IntMap;

fn main() {
    println!("Rock, Paper, Scissors");
    println!("Please input your object. Or type 'exit' to quit.");
    let mut user_score = 0;
    let mut computer_score = 0;
    let mut ties_counter = 0;

    loop {
        let mut user_object = String::new();
        let mut computer_object = String::new();

        io::stdin().read_line(&mut user_object)
            .expect("Failed to read line");

        let user_object = user_object.trim();

        println!("You entered: {}", user_object);

        if user_object == "rock" {
            computer_object = chose_random_game_item();
            println!("Computer chooses {}", computer_object);
            println!("The winner is: {}", who_wins(user_object, &computer_object))
        } else if user_object == "paper" {
            computer_object = chose_random_game_item();
            println!("Computer chooses {}", computer_object);
            println!("The winner is: {}", who_wins(user_object, &computer_object))
        } else if user_object == "scissors" {
            computer_object = chose_random_game_item();
            println!("Computer chooses {}", computer_object);
            println!("The winner is: {}", who_wins(user_object, &computer_object))
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

fn who_wins(user_object: &str, computer_object: &str) -> String {
    let objects_map: HashMap<&str, usize> = [("rock", 0), ("paper", 1), ("scissors", 2)].iter().cloned().collect();
    let user_choice = objects_map[&user_object];
    let computer_choice = objects_map[&computer_object];
    let x = [
        ["tie", "computer", "user"],
        ["user", "tie", "computer"],
        ["computer", "user", "tie"],
    ];
    x[user_choice][computer_choice].to_string()
}
