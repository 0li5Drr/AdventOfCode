use std::collections::HashMap;
use std::fs;
use camino::Utf8PathBuf;

/* First thoughts are to break into 3 Steps:
    1. Create a dummy file system (tree?)
    2. Parse instructions and write size into file
    3. Iterate through each directory
*/

//For Command Processing - inspired by fasterthanli.me solution

#[derive()]
enum InputLine {
    Command(Command),
    Entry(Entry),
}

#[derive()]
enum Entry {
    Dir(Utf8PathBuf),
    File(u64, Utf8PathBuf),
}


#[derive()]
struct Cd(Utf8PathBuf);

fn parse_cmd(i:&str) -> Option<Command> {
    if i.starts_with("cd") {
        if let Some(cd) = parse_cd(i) {
            return Some(Command::Cd(cd));
        }
    }
    else if i.starts_with("ls") { return Some(Command::Ls) }
    None
}

fn parse_cd(i : &str) -> Option<Cd>{
    if let Some(p) = i.strip_prefix("cd") {
        let path = Utf8PathBuf::from(p);
        return Some(Cd(path));
    }
    None
}

fn parse_entry(i : &str) -> Entry {
    if let Some(v) = i.strip_prefix("dir ") {
        return Entry::Dir(Utf8PathBuf::from(v))
    }
    let (size, path) = i.split_once(" ").unwrap(); //Entry will always have a space
    let size : u64 = size.parse().unwrap();
    let path = Utf8PathBuf::from(path);
    Entry::File(size, path)
}

fn parse_line(i : &str) -> Option<InputLine> {

    match i.strip_prefix("$") {
        Some(c) => {
            if let Some(cmd) = parse_cmd(c) {
                return Some(cmd);
            }
        }
        None => {
            if let Some(e) = parse_entry(i) {
                return Some(e)
            }
        }
    }
    None
}

#[derive()]
enum Command {
    Cd(Cd),
    Ls,
}

//For Dummy File Tree
#[derive()]
struct FileNode {
    size : usize,
    children : HashMap<Utf8PathBuf, FileNode>
}

fn main() {
    println!("Hello, world!");
}
