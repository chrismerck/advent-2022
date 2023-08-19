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
use std::rc::{Rc, Weak};

struct TreeNode<T> {
    data: T,
    parent: Option<Weak<TreeNode<T>>>,
    children: Vec<Rc<TreeNode<T>>>,
}

impl<T> TreeNode<T> {
    fn new(data: T) -> Self {
        Self {
            data,
            parent: None,
            children: Vec::new(),
        }
    }

    fn insert(&mut self, data: T) {
        let mut new_node = TreeNode::new(data);
        let new_node_rc = Rc::new(new_node);
        new_node_rc.parent = Some(Rc::downgrade(&self.rc()));
        self.children.push(new_node_rc);
    }

    fn search(&self, data: T) -> Option<Rc<TreeNode<T>>> {
        if self.data == data {
            return Some(self.rc());
        }

        for child in &self.children {
            let result = child.search(data);
            if result.is_some() {
                return result;
            }
        }

        return None;
    }

    fn rc(&self) -> Rc<TreeNode<T>> {
        Rc::new(self.clone())
    }
}

struct File {
    name: String,
    size: u64,
}

struct FileSystem {
    root: TreeNode<File>,
}

/*
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
}*/

impl FileSystem {
    fn new() -> Self {
        Self {
            root: TreeNode::new(File {
                name: "/".to_string(),
                size: 0,
            }),
        }
    }

    fn add_file(&mut self, path: &str, size: u64) {

    fn get_size_recursive(&self, path: &str) -> u64 {
        let mut path_parts = path.split("/");
        let mut current_node = &self.root;
        for part in path_parts {
            if part == "" {
                continue;
            }
            let mut found = false;
            for child in &current_node.children {
                if child.data.name == part {
                    current_node = child;
                    found = true;
                    break;
                }
            }
            if !found {
                return 0;
            }
        }
        current_node.data.size
    }

    fn print(&self) {
        self.root.print(0);
    }
}

// read from file "input" and build tree
fn parse_input(lines: &[String]) -> Node {
    let mut root = Node::new("/".to_string(), 0);
    let mut last_cmd = "".to_string();
    let mut breadcrumbs = Vec::new();
    breadcrumbs.push(&mut root);
    for line in lines {
        parse_line(line, &mut last_cmd, &mut breadcrumbs);
    }
    root
}

fn parse_line(line: &String, last_cmd: &mut String, breadcrumbs: &mut Vec<&mut Node>) {
    // use subroutines for each command
    let mut words = line.split_whitespace();
    let first = words.next().unwrap();
    match first {
        "$" => {
            parse_cmd(&mut words, last_cmd, breadcrumbs);
        },
        _ => {
            parse_output(&mut words, last_cmd, breadcrumbs);
        }
    }
}

fn parse_cmd(words: &mut dyn Iterator<Item=&str>, last_cmd: &mut String, breadcrumbs: &mut Vec<&mut Node>) {
    let second = words.next().unwrap();
    *last_cmd = second.to_string();
    match second {
        "cd" => {
            parse_cd(words, breadcrumbs);
        },
        "ls" => {
            parse_ls(words, breadcrumbs);
        },
        _ => {
            println!("WARNING: unknown command {}", second);
        }
    }
}

fn parse_cd(words: &mut dyn Iterator<Item=&str>, breadcrumbs: &mut Vec<&mut Node>) {
    let third = words.next().unwrap();
    let mut cwd = breadcrumbs.pop().unwrap();
    match third {
        "/" => {
            println!("cmd: cd root");
            if cwd.name != "/" {
                println!("WARNING: cd to root unimplemented, ignoring");
            }
            breadcrumbs.push(cwd);
        }
        ".." => {
            println!("cmd: cd ..");
            /* go up one level by popping back up one level in the call stack */
        }
        _ => {
            println!("cmd: cd {}", third);
            /* go down one level by pushing down one level in the call stack */
            let mut found = false;
            for child in &mut cwd.children {
                if child.name == third {
                    breadcrumbs.push(cwd);
                    breadcrumbs.push(child);
                    found = true;
                    break;
                }
            }
        }
    }
}

fn parse_ls(words: &mut dyn Iterator<Item=&str>, breadcrumbs: &mut Vec<&mut Node>) {
    println!("cmd: ls");
    /* as there are no arguments, and the replies come on later lines,
        we can just ignore this command other than remembering that it was last run */
}

fn parse_output(words: &mut dyn Iterator<Item=&str>, last_cmd: &mut String, breadcrumbs: &mut Vec<&mut Node>) {
    match &last_cmd[..] {
        "ls" => {
            parse_ls_output(words, breadcrumbs);
        },
        _ => {
            println!("WARNING: unexpected output for cmd {}", &last_cmd);
        }
    }
}

fn parse_ls_output(words: &mut dyn Iterator<Item=&str>, breadcrumbs: &mut Vec<&mut Node>) {
    let mut size = 0;
    let name;
    let first = words.next().unwrap();
    if first != "dir" {
        size = first.parse::<u64>().unwrap();
    }
    name = words.next().unwrap();
    let mut found = false;
    for child in &mut breadcrumbs.last().unwrap().children {
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
        breadcrumbs.last().unwrap().add_child(new_file);
    }
}

fn main() {
    // read from file called "input.txt" into vector of lines
    let mut lines = Vec::new();
    let file = File::open("input.example").unwrap();
    for line in io::BufReader::new(file).lines() {
        lines.push(line.unwrap());
    }

    // build tree 
    let root = parse_input(&lines);

    // print it
    root.print(0);
}
