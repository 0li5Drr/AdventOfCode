fn main() {
    let mut counter : u32 = 0;
    let pairs = include_str!("input.txt").lines().collect::<Vec<&str>>();
    for pair in pairs {
        let elves : Vec<&str> = pair.split(",").collect();

        let mut elf_range : Vec<&str> = elves[0].split("-").collect();
        let e1_low :u32 = elf_range[0].trim().parse().unwrap();
        let e1_upper:u32 = elf_range[1].trim().parse().unwrap();
        elf_range = elves[1].split("-").collect();
        let e2_low : u32 = elf_range[0].trim().parse().unwrap();
        let e2_upper:u32 = elf_range[1].trim().parse().unwrap();

        if (e1_low <= e2_low && e1_upper >= e2_upper) || (e1_low >= e2_low && e1_upper <= e2_upper) {
            counter += 1;
        } 
    }
    println!("Part 1: There are {counter} elves whose range is included within their partner's. \n");
}
