use std::collections::HashSet;

// struct rucksack contains two compartments
// each compartment can hold any number of items
// each item is represented by a char
// rucksacks also have a unique identifier
struct Rucksack {
    left: Vec<char>,
    right: Vec<char>,
    uid: u64,
}

// implement the Rucksack struct
impl Rucksack {
    // create a new rucksack with no items
    // and a random uid
    fn new() -> Rucksack {
        Rucksack {
            left: Vec::new(),
            right: Vec::new(),
            uid: rand::random(),
        }
    }

    // version of constructor that takes a string
    // and adds each char to the rucksack
    // and then balances the rucksack
    fn from_string(s: &str) -> Rucksack {
        let mut rucksack = Rucksack::new();
        for c in s.chars() {
            rucksack.add(c);
        }
        rucksack.balance();
        rucksack
    }

    // add an item to the rucksack
    // this always goes into the left compartment
    fn add(&mut self, item: char) {
        self.left.push(item);
    }

    // balance the rucksack
    // this moves the latter half of the items from
    // the left to the right compartment
    fn balance(&mut self) {
        assert!(self.right.is_empty());
        assert!(self.left.len() % 2 == 0);
        let midpoint = self.left.len() / 2;
        self.right = self.left.split_off(midpoint);
    }

    // find common items
    // returns a set of items that are in both compartments
    // (No duplicates in returned set.)
    // (case-sensitive)
    fn find_common(&self) -> HashSet<char> {
        let mut common = HashSet::new();
        for item in &self.left {
            if self.right.contains(item) {
                common.insert(*item);
            }
        }
        common
    }

    // print the rucksack
    fn print(&self) {
        println!("Rucksack #{}", self.uid);
        print!("  Left:  ");
        for item in &self.left {
            print!("{}", item);
        }
        println!();
        print!("  Right: ");
        for item in &self.right {
            print!("{}", item);
        }
        println!();
    }
}

// equality operator for rucksacks
// rucksacks are equal if they have the same uid
impl PartialEq for Rucksack {
    fn eq(&self, other: &Self) -> bool {
        self.uid == other.uid
    }
}

impl Eq for Rucksack {}

// get priority of item
// a--z: 1--26
// A--Z: 27--52
fn get_priority(item: char) -> u32 {
    let mut priority = 0;
    if item.is_ascii_lowercase() {
        priority = item as u32 - 96;
    } else if item.is_ascii_uppercase() {
        priority = item as u32 - 64 + 26;
    }
    assert!(priority > 0);
    priority
}

// load rucksacks from file
// one line per rucksack
// each line contains a string of items
fn load_rucksacks(filename: &str) -> Vec<Rucksack> {
    let mut rucksacks = Vec::new();
    let contents = std::fs::read_to_string(filename).expect("Error reading file");
    for line in contents.lines() {
        rucksacks.push(Rucksack::from_string(line));
    }
    rucksacks
}

// get an arbitrary item from a set
fn get_item(set: &HashSet<char>) -> char {
    let mut item = '?';
    for i in set {
        item = *i;
        break;
    }
    item
}

fn main() {
    // check that get_priority works
    assert_eq!(get_priority('a'), 1);
    assert_eq!(get_priority('z'), 26);
    assert_eq!(get_priority('A'), 27);
    assert_eq!(get_priority('Z'), 52);

    // load rustsacks from file
    let rucksacks = load_rucksacks("input");

    // print the rucksacks
    for rucksack in &rucksacks {
        rucksack.print();
    }
    
    // find common items in each rucksack
    // asserting that there be exactly one common item in each rucksack
    for rucksack in &rucksacks {
        let common = get_item(&rucksack.find_common());
        println!("Rucksack #{} common item = {}", rucksacks.iter().position(|r| r == rucksack).unwrap() + 1, common);
    }

    // compute the sum of the priorities of the common items in each rucksack
    let mut sum = 0;
    for rucksack in &rucksacks {
        let common = get_item(&rucksack.find_common());
        sum += get_priority(common);
    }

    // print the sum
    println!("Sum of left-right common item priorities = {}", sum);
}
