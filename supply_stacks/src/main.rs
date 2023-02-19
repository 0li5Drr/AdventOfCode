
const NUMBER_OF_STACKS :u32 = 9;

fn main() {
    //Initialising
    let input = include_str!("input.txt");
    let (input, instructions) = input.split_once("\n\r").expect("Failed to split input text properly. Check formatting or correct input is given.\n");
    let instructions  = instructions.lines();
    let mut input = input.lines().rev();
    let mut stacks = Vec::new();
    for _ in 0..NUMBER_OF_STACKS {
        let mut vec : Vec<&str> = Vec::new();
        stacks.push(vec);
    }
    // Now to read backwards
    let _ = input.next().expect("Parsing Error, no stacks found.");
    for line in input {
        let mut idx = 1;
        //Iterate for each Stack:
        for stack in stacks.iter_mut() {
            match line.get(idx..idx+1) {
                None => {}
                Some(val) => {
                    //println!("Stack: {}, Value: {val}", (idx - 1)/4 +1);
                    stack.push(val);
                }
            }
            idx += 4;
        }

    }
}
