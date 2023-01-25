


fn main() {
    let args = std::env::args().collect();
    print_usage_and_exit()
}

fn print_usage_and_exit() {
    println!("cargo run --release [path]\npath being the path to an input txt file.")
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}