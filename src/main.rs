use std::io;
use rand::Rng;
use std::collections::HashMap;
use intmap::IntMap;

fn main() {
    println!("Rock, Paper, Scissors");
    println!("Please input your object. Or type 'exit' to quit.");
    let mut user_score = &mut 0;
    let mut computer_score = &mut 0;
    let mut ties_counter = &mut 0;
    let mut games_counter = 0;

    loop {
        let mut user_object = String::new();
        let mut computer_object = String::new();

        io::stdin().read_line(&mut user_object)
            .expect("Failed to read line");

        let user_object = user_object.trim();

        println!("You entered: {}", user_object);

        if user_object == "rock" || user_object == "paper" || user_object == "scissors" {
            computer_object = chose_random_game_item();
            let mut winner = String::new();
            println!("Computer chooses: {}", computer_object);
            winner = who_wins(user_object, &computer_object);
            update_scores(&winner, user_score, computer_score, ties_counter);
            games_counter = games_counter + 1;
            println!("The winner is: {}", winner);
            println!("The scores over {} games are > user: {}, computer: {} and {} ties", games_counter, user_score, computer_score, ties_counter);
        } else if user_object == "exit" {
            println!("Thanks for playing, bye!");
            break;
        } else {
            println!("Hmm ... I didn't get that, please enter 'rock', 'paper' or 'scissors'");
            continue;
        }
    }
}

fn update_scores(result: &String, user_score: &mut i32, computer_score: &mut i32, ties_counter: &mut i32) -> () {
    if result == "user" {
        *user_score = *user_score + 1;
    } else if result == "computer" {
        *computer_score = *computer_score + 1;
    } else {
        *ties_counter = *ties_counter + 1;
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
