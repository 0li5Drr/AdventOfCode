use std::io::{self, BufRead};
use std::fs::File;
use std::path::Path;


fn main() {
    let args : Vec<String> = std::env::args().skip(1).collect();
    let rucksacks = read_lines(&args[0]).expect("Erorr reading file");

     //Part 1
    //This is absolutely revolting but I wanted to try using the fold operator, and couldnt figure out how to make it compile any other way.
    let mut priority_sum = rucksacks.fold(0,|acc, rucksack| 
        {let mut new_acc = acc;
        if let Ok(rksk) = rucksack {
            if let Some(prio) = get_priority(Some(find_duplicate(&rksk))) {new_acc += prio};
        }; 
        new_acc});
    
    println!("Part 1 - Total Priority = {priority_sum}\n");
        
    //Part 2 - Rucksacks in Groups of 3 - Common item between all 3 is Badge - find sum of Badge Items.

    //In researching, I found the include_str! macro, which while simpler, is obviously somewhat suboptimal to store the String in memory if it gets really big.
    let rucksacks  = include_str!("../input.txt").lines().collect::<Vec<_>>();
    priority_sum = 0;
    let num_groups = rucksacks.len() / 3;
    for idx in 0..num_groups {
        priority_sum += get_priority(find_badge(&rucksacks.get(idx*3).unwrap(),&rucksacks.get(idx*3 +1).unwrap(),&rucksacks.get(idx*3 +2).unwrap())).unwrap();
        //Error handling is apparently a lie 
    }
    println!("Part 2 - Badge Priority Sum: {priority_sum}\n");
            

}

fn find_badge(rs1: &str, rs2: &str, rs3: &str) -> Option<char> {
    for item in rs1.chars() {
        if rs2.contains(item) && rs3.contains(item){
            return Some(item);
        }
    }
    return None;
}

fn find_duplicate(rucksack : &str) -> char {
    let (compt_1, compt_2) = rucksack.split_at(rucksack.len() / 2);
    for item in compt_1.chars() {
        if compt_2.contains(item){
            return item;
        }
    }
    return '\0';
}

fn get_priority(item : Option<char>) -> Option<i32>{
    //I have decided to just use a lookup table to match values, because I haven't yet figured out how to work
    if let Some(item) = item {
            let val:i32 = match item.to_lowercase().nth(0).unwrap() {
            'a' => 1,
            'b' => 2,
            'c' => 3,
            'd' => 4,
            'e' => 5,
            'f' => 6,
            'g' => 7,
            'h' => 8,
            'i' => 9,
            'j' => 10,
            'k' => 11,
            'l' => 12,
            'm' => 13,
            'n' => 14,
            'o' => 15,
            'p' => 16,
            'q' => 17,
            'r' => 18,
            's' => 19,
            't' => 20,
            'u' => 21,
            'v' => 22,
            'w' => 23,
            'x' => 24,
            'y' => 25,
            'z' => 26,
            _ => -200
        };
        if val < 0 {
            return None;
        }
        if item.is_uppercase() {
            return Some(26+val);
        }
        return Some(val);
    }
    return None;
    
}
//Function I have gotten from somewhere to read all lines in a file, but don't know author anymore.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

