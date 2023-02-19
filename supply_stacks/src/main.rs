
const NUMBER_OF_STACKS :u32 = 9;

fn main() {
    part_one();
    part_two();
}

fn part_two() {
    //Initialising
    let input = include_str!("input.txt");
    let (input, instructions) = input.split_once("\n\r").expect("Failed to split input text properly. Check formatting or correct input is given.\n");
    let mut instructions  = instructions.lines();
    let mut input = input.lines().rev();
    let mut stacks = Vec::new();
    for _ in 0..NUMBER_OF_STACKS {
        let vec : Vec<&str> = Vec::new();
        stacks.push(vec);
    }
    // Now to read initial stack state and initialise as such.
    let _ = input.next().expect("Parsing Error, no stacks found.");
    for line in input {
        let mut idx = 1;
        //Iterate for each Stack:
        for stack in stacks.iter_mut() {
            match line.get(idx..idx+1) {
                None => {}
                Some(" ") => {}
                Some(val) => {
                    //println!("Stack: {}, Value: {val}", (idx - 1)/4 +1);
                    stack.push(val);
                }
            }
            idx += 4; //Number of characters before next
        }
    }
    instructions.next().unwrap();
    //Now to work through Instructions:
    for instruction in instructions {
        let mut words = instruction.split(" ");
        let (_, mut num_objs, _, mut origin, _, mut dest) =
            (words.next(), words.next(), words.next(), words.next(), words.next(), words.next());
        //println!("Move {} from {} to {}", num_objs.get_or_insert("ERR"), origin.get_or_insert("ERR"), dest.get_or_insert("ERR"));


        let num_objs : u32 = num_objs.get_or_insert("ERR").parse().expect("Failed to parse number of crates to move as number.");
        let mut origin : usize = origin.get_or_insert("ERR").parse().expect("Failed to parse original stack as number.");
        let mut dest : usize = dest.get_or_insert("ERR").parse().expect("Failed to parse destination stack to move as number.");
        origin -= 1;
        dest -= 1;
        let stack_vec = &mut stacks;
        let mut crane_vec : Vec<&str> = vec!();
        for _  in 0..num_objs {
            match stack_vec[origin].pop() {
                None => {}
                Some(v) => {crane_vec.push(v)}
            }

        }
        for _ in 0..num_objs {
            match crane_vec.pop() {
                None => {}
                Some(v) => {stack_vec[dest].push(v);}
            }
        }
    }
    //Now we simply read the top crate off each stack.
    let mut i : u8 = 0;
    for stack in &mut stacks {
        match stack.last() {
            None => {}
            Some(v) => {println!("Stack {}: {v}", i+1)}
        }
        i += 1;
    }
    println!("\n");
}

fn part_one() {
    //Initialising
    let input = include_str!("input.txt");
    let (input, instructions) = input.split_once("\n\r").expect("Failed to split input text properly. Check formatting or correct input is given.\n");
    let mut instructions  = instructions.lines();
    let mut input = input.lines().rev();
    let mut stacks = Vec::new();
    for _ in 0..NUMBER_OF_STACKS {
        let vec : Vec<&str> = Vec::new();
        stacks.push(vec);
    }
    // Now to read initial stack state and initialise as such.
    let _ = input.next().expect("Parsing Error, no stacks found.");
    for line in input {
        let mut idx = 1;
        //Iterate for each Stack:
        for stack in stacks.iter_mut() {
            match line.get(idx..idx+1) {
                None => {}
                Some(" ") => {}
                Some(val) => {
                    //println!("Stack: {}, Value: {val}", (idx - 1)/4 +1);
                    stack.push(val);
                }
            }
            idx += 4; //Number of characters before next
        }
    }
    instructions.next().unwrap();
    //Now to work through Instructions:
    for instruction in instructions {
        let mut words = instruction.split(" ");
        let (_, mut num_objs, _, mut origin, _, mut dest) =
            (words.next(), words.next(), words.next(), words.next(), words.next(), words.next());
        //println!("Move {} from {} to {}", num_objs.get_or_insert("ERR"), origin.get_or_insert("ERR"), dest.get_or_insert("ERR"));


        let num_objs : u32 = num_objs.get_or_insert("ERR").parse().expect("Failed to parse number of crates to move as number.");
        let mut origin : usize = origin.get_or_insert("ERR").parse().expect("Failed to parse original stack as number.");
        let mut dest : usize = dest.get_or_insert("ERR").parse().expect("Failed to parse destination stack to move as number.");
        origin -= 1;
        dest -= 1;
        let stack_vec = &mut stacks;
        for _  in 0..num_objs {
            match stack_vec[origin].pop() {
                None => {}
                Some(v) => {stack_vec[dest].push(v)}
            }

        }
    }
    //Now we simply read the top crate off each stack.
    let mut i : u8 = 0;
    for stack in &mut stacks {
        match stack.last() {
            None => {}
            Some(v) => {println!("Stack {}: {v}", i+1)}
        }
        i += 1;
    }
    println!("\n");
}