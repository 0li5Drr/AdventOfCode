use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.is_empty() {
        print_usage_and_exit();
        return;
    }
    
    let path = &args[0];
    let contents = read_lines(&path).expect("Failed to read file");
    
    let mut elves : Vec<i32> = Vec::new();
    let mut mostCalories :i32 = 0;
    let mut currentSum :i32 = 0;

    for line in contents {
        if let Ok(calories) = line {
            if calories.eq("") {
                if currentSum > mostCalories {
                    mostCalories = currentSum;
                    
                }
                elves.push(currentSum);
                currentSum = 0;
            } else {
                if let Ok(cals) = str::parse::<i32>(&calories) {
                    currentSum += cals;
                    
                };
            }
        }
    }
    elves.sort();
    elves = elves.into_iter().rev().collect();

    println!("Top 3 Elves: {}, {}, {}\nSum of Top 3: {}", elves.get(0).unwrap(), elves.get(1).unwrap(), elves.get(2).unwrap(), (elves.get(0).unwrap() + elves.get(1).unwrap() + elves.get(2).unwrap()));
}

fn print_usage_and_exit() {
    println!("Only one argument used: Path of File to .txt");
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}