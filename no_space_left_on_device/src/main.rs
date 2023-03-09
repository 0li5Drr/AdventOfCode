use std::borrow::Borrow;
use std::collections::HashMap;
use std::fs;
use std::fs::File;
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
    let size : u64 = size.trim().parse().unwrap();
    let path = Utf8PathBuf::from(path);
    Entry::File(size, path)
}

fn parse_line(i : &str) -> InputLine {
    if let Some(c) = i.strip_prefix("$") {
        if let Some(cmd) = parse_cmd(c) {
            return InputLine::Command(cmd);
        }
    }
    InputLine::Entry(parse_entry(i))
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

// Now that Structs and Enums are sorted, will take a breadcrumb stack approach to paths?

fn main() {
    let mut pwd = Utf8PathBuf::from("/");
    let mut root :FileNode = FileNode { size: 0, children: HashMap::new() };
    let mut breadcrumbs: Vec<&mut FileNode> = vec![&mut root]; //Last is always current FileNode

    //This is where initialise the dummy system by parsing all lines.
    let lines : Vec<&str> = include_str!("input.txt").lines().collect();
    for line in lines {
        match parse_line(line) {
            InputLine::Command(c) => {
                match c {
                    Command::Cd(cd) => {
                        if cd.0.eq("..") {
                            breadcrumbs.pop();
                        } else {
                            pwd = pwd.join(&cd.0);
                            let current = breadcrumbs.last().unwrap();
                            let child = current.children.get_mut(&cd.0).unwrap();
                            breadcrumbs.push(child);
                        }
                    },
                    Command::Ls => {/*No need to do anything*/}
                }
            },
            InputLine::Entry(e) => {
                match e {
                    Entry::Dir(p) => {
                        let mut children = &mut breadcrumbs.last_mut().unwrap().children;
                        children.insert(p, FileNode{size : 0, children : HashMap::new()});

                    },
                    Entry::File(s,p) => {}
                }
            },
        }
    }
}
