use std::cmp;


fn main() {
    let mut counter : u32 = 0;
    let mut total_overlap : u32 = 0;
    let mut overlapping_pairs : u32 = 0;
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
        if overlaps(e1_low, e1_upper, e2_low, e2_upper) {
            overlapping_pairs += 1;
        }

        total_overlap += overlap_size(e1_low, e1_upper, e2_low, e2_upper);
    }
    println!("Part 1: There are {counter} elves whose range is included within their partner's. \n");
    println!("Part 2 if you don't read the exercise: The total overlap is {total_overlap} sectors.\n");

    println!("Actual Part 2: There are {} pairs which overlap sectors.\n", overlapping_pairs);
}
/// 
fn overlap_size(e1_lo: u32, e1_hi : u32, e2_lo :u32, e2_hi :u32) -> u32 {
    //No overlap
    if !(e1_lo <= e2_hi && e2_lo <= e1_hi) {
        return 0;
    } 
    
    let mut overlap :u32 = 0;
    let mut i :u32 = 0;

    let lo : u32 = cmp::max(e1_lo, e2_lo);
    let hi : u32 = cmp::min(e1_hi, e2_hi);

    while lo + i <= hi {
        overlap += 1;
        i += 1;
    }

    return overlap
}
fn overlaps(e1_lo: u32, e1_hi : u32, e2_lo :u32, e2_hi :u32) -> bool {
    return e1_lo <= e2_hi && e2_lo <= e1_hi;
}