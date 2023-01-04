/*
The task here is to build a tree representing a directory
structure which we learn from a terminal log (input).
We will then be able to query the tree for information
about the size of a directory (which recursively sums the
sizes of all files and subdirectories).

Input example:

"$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k"
*/

use std::io::{self, BufRead};
use std::fs::File;


struct Node {
    name: String,
    size: u64,
    children: Vec<Node>,
}

impl Node {
    fn new(name: String, size: u64) -> Node {
        Node {
            name,
            size,
            children: Vec::new(),
        }
    }

    fn add_child(&mut self, child: Node) {
        self.children.push(child);
    }

    fn get_size_recursive(&self) -> u64 {
        let mut size = self.size;
        for child in &self.children {
            size += child.get_size_recursive();
        }
        size
    }

    fn print(&self, indent: u32) {
        for _ in 0..indent {
            print!(" ");
        }
        println!("{} {} {}", self.name, self.size, self.get_size_recursive());
        for child in &self.children {
            child.print(indent + 1);
        }
    }
}

// read from file "input" and build tree
/*

Input example:

"$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k"
*/

/* recursively build tree */
// include prints for debugging
fn build_tree(lines: &[String], current_dir: &mut Node, mut last_cmd: String) {
    // null case
    if lines.len() == 0 {
        return;
    }
    let mut words = lines[0].split_whitespace();
    let first = words.next().unwrap();
    match first {
        "$" => {
            let second = words.next().unwrap();
            last_cmd = second.to_string();
            match second {
                "cd" => {
                    let third = words.next().unwrap();
                    match third {
                        "/" => {
                            println!("cmd: cd root");
                            if current_dir.name != "/" {
                                println!("WARNING: cd to root unimplemented, ignoring");
                            }
                        }
                        ".." => {
                            println!("cmd: cd ..");
                            /* go up one level by popping back up one level in the call stack */
                            return;
                        }
                        _ => {
                            println!("cmd: cd {}", third);
                            /* recurse into subdirectory */
                            let mut found : Option<&mut Node> = current_dir.children.iter_mut().find(|child| child.name == third);
                            if found.is_none() {
                                println!("WARNING: directory {} not found, creating implicitly", third);
                                let new_dir = Node::new(third.to_string(), 0);
                                current_dir.add_child(new_dir);
                                found = current_dir.children.iter_mut().find(|child| child.name == third);
                            }
                            build_tree(&lines[1..], found.unwrap(), last_cmd);
                            return;
                        }
                    }
                }
                "ls" => {
                    println!("cmd: ls");
                    /* as there are no arguments, and the replies come on later lines,
                        we can just ignore this command other than remembering that it was last run */
                }
                _ => {
                    println!("WARNING: unknown command {}", second);
                }
            }
        }
        _ => {
            match &last_cmd[..] {
                "ls" => {
                    let mut size = 0;
                    let name;
                    if first == "dir" {
                        name = words.next().unwrap();
                    } else {
                        size = first.parse::<u64>().unwrap();
                        name = words.next().unwrap();
                    }
                    let mut found = false;
                    for child in &mut current_dir.children {
                        if child.name == name {
                            println!("WARNING: {} {} already exists, updating size", 
                                match size { 0 => "dir", _ => "file" }, name);
                            child.size = size;
                            found = true;
                            break;
                        }
                    }
                    if !found {
                        let new_file = Node::new(name.to_string(), size);
                        current_dir.add_child(new_file);
                    }
                }
                _ => {
                    println!("WARNING: unexpected output for cmd {}", &last_cmd);
                }
            }
        }
    }
    build_tree(&lines[1..], current_dir, last_cmd);
}

fn main() {
    // read from file called "input.txt" into vector of lines
    let mut lines = Vec::new();
    let file = File::open("input.example").unwrap();
    for line in io::BufReader::new(file).lines() {
        lines.push(line.unwrap());
    }

    // build tree
    let mut root = Node::new("/".to_string(), 0);
    let current_dir = &mut root;
    build_tree(&lines, current_dir, "".to_string());

    // print it
    root.print(0);
}
