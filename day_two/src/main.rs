use std::{fs, fmt::Result};

//Opponent
//A rock
//B paper
//C scissors
//Me
//X rock 1p
//Y paper 2p
//Z scissors 3p
//
//Points
//L 0
//D 3
//W 6

fn main() {
    let results = read_puzzle_input("./dtwo_data.txt");
    let score = calculate_score(results);

    println!("Total score for me: {:?}", score)
}


fn calculate_score(results: Vec<String>) -> i64 {
    let mut total_score = 0;

    for result in results.iter() {
        let split: Vec<&str> = result.split(" ").collect();
        let tuple: (&str, &str) = (split[0], split[1]); 
        total_score += handle_round(tuple);
    }
    
    total_score
}

fn handle_round(round: (&str, &str)) -> i64 {
    let mut round_sum = 0;

    if round.0 == "A" {
        //W
        if round.1 == "Y" {
            round_sum = 8
        } 
        //D
        else if round.1 == "X" {
            round_sum = 4
        }
        //L
        else {
            round_sum = 3
        }
    } else if round.0 == "B" {
        //W
        if round.1 == "Z" {
            round_sum = 3 + 6
        }
        //D
        else if round.1 == "Y" {
            round_sum = 2 + 3
        }
        //L
        else {
            round_sum = 1 + 0
        }
    } else {
        //W
        if round.1 == "X" {
            round_sum = 1 + 6
        }
        //D
        else if round.1 == "Z" {
            round_sum = 3 + 3
        }
        //L
        else {
            round_sum = 2 + 0
        }
    }
    
    round_sum
}


fn read_puzzle_input(path: &str) -> Vec<String> {
    //Read the file as stream and create a vector
    let stream: String = fs::read_to_string(path).expect("Error reading file");

    let mut results: Vec<String> = Vec::new();

    //Split the stream \n and add each element to results vector
    for elem in stream.split("\n") {
        results.push(elem.to_string())
    }

    //Return
    results
}