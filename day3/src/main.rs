use std::collections::HashSet;

// struct rucksack contains two compartments
// each compartment can hold any number of items
// each item is represented by a char
// rucksacks also have a unique identifier
struct Rucksack {
    left: Vec<char>,
    right: Vec<char>,
    uid: u64,
    group: u32,
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
            group: 0,
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

    // convert a rucksack to a set of items
    // without duplicates
    // combining items from both compartments
    fn to_set(&self) -> HashSet<char> {
        let mut set = HashSet::new();
        for item in &self.left {
            set.insert(*item);
        }
        for item in &self.right {
            set.insert(*item);
        }
        set
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

// organize a list of rucksacks into groups
// return a dictionary mapping group id to list of rucksacks in that group
// do not clone the rucksacks
fn organize(rucksacks: &Vec<Rucksack>) -> std::collections::HashMap<u32, Vec<&Rucksack>> {
    let mut groups = std::collections::HashMap::<u32, Vec<&Rucksack>>::new();
    for rucksack in rucksacks {
        let group = rucksack.group;
        if groups.contains_key(&group) {
            groups.get_mut(&group).unwrap().push(rucksack);
        } else {
            groups.insert(group, vec![rucksack]);
        }
    }
    groups
}

// find intersection of all rucksacks in a group
fn intersect_group(rucksacks: &Vec<&Rucksack>) -> HashSet<char> {
    // apply bitwise and operator to all rucksacks in group
    let mut intersection = rucksacks[0].to_set();
    for rucksack in rucksacks {
        intersection = intersection
            .intersection(&rucksack.to_set())
            .map(|&x| x)
            .collect();
    }
    intersection
}

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
    let mut group = 0;
    let mut elf_in_group = 0;
    for line in contents.lines() {
        rucksacks.push(Rucksack::from_string(line));
        rucksacks.last_mut().unwrap().group = group;
        elf_in_group += 1;
        if elf_in_group == 3 {
            elf_in_group = 0;
            group += 1;
        }
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

    // organize rucksacks into groups
    let groups = organize(&rucksacks);

    // find intersection of each group
    let mut sum = 0;
    for group in groups.values() {
        let intersection = intersect_group(group);
        let item = get_item(&intersection);
        println!("Group {} intersection = {}", group[0].group, item);
        sum += get_priority(item);
    }

    // print the sum
    println!("Sum of badge priorities = {}", sum);
}
