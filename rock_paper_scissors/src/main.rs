use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let args : Vec<String> = std::env::args().skip(1).collect();
    if args.len() == 0 {
        print_usage_and_exit();
    }
    let lines = read_lines(&args[0]).expect("Erorr reading file");
    let mut score : i32 = 0;
    let mut score2 : i32 = 0;

    for line in lines {
        if let Ok(strategy) = line {
            score += score_strategy_part_one(strategy.chars().nth(0).unwrap(), strategy.chars().nth(2).unwrap());
            score2 += score_strategy_part_two(strategy.chars().nth(0).unwrap(), strategy.chars().nth(2).unwrap());
        }
    }
    println!("Final Strategy Score Part 1 = {}", score);
    println!("Final Strategy Score Part 2 = {}", score2);
}

fn score_strategy_part_one(opponent : char, player :char) -> i32{
    //A,X Rock ; B,Y Paper; C,Z Scissors
    match (opponent, player) {
        ('A', 'X') => 1 + 3, //Rock, Rock - Tie
        ('A', 'Y') => 2 + 6, // Rock, Paper - Win
        ('A', 'Z') => 3, //Rock, Scissors
        ('B', 'X') => 1, //Paper, Rock
        ('B', 'Y') => 2 + 3, //Paper, Paper - Tie
        ('B', 'Z') => 3 + 6, //Paper, Scissors - Win
        ('C', 'X') => 1 + 6, //Scissors, Rock - Win
        ('C', 'Y') => 2, //Scissors, paper
        ('C', 'Z') => 3 + 3, //Scissors, Scissors - Tie
        (_, _) => {println!("Invalid Character Combination! Returned 0, but please ignore result!"); 0} 
    }
}

fn score_strategy_part_two(opponent : char, outcome :char) -> i32{
    //A,X Rock ; B,Y Paper; C,Z Scissors
    match (opponent, outcome) {
        ('A', 'X') => 0 + 3, //Loss - Scissors
        ('A', 'Y') => 3 + 1, //Tie - ROck
        ('A', 'Z') => 6 + 2, //Win - Paper
        ('B', 'X') => 0 + 1, //Lose - Rock
        ('B', 'Y') => 3 + 2, //Tie - Paper
        ('B', 'Z') => 6 + 3, //Win - Scissors
        ('C', 'X') => 0 + 2, //Loss - Paper
        ('C', 'Y') => 3 + 3, //Tie - Scissors
        ('C', 'Z') => 6 + 1, //Win - Rock
        (_, _) => {println!("Invalid Character Combination! Returned 0, but please ignore result!"); 0} 
    }
}

fn print_usage_and_exit() {
    println!("cargo run --release [path]\npath being the path to an input txt file.")
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}