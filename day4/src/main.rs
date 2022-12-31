/*
 * Input file format is many lines containing a pair of assignments of the form:
 *
 *  [\d]+-[\d]+,[\d]+-[\d]+
 *
 * The first range is the section assignment for the first elf.
 * The second range is the section assignment for the second elf in a pair.
 *
 * Our goal is to find the number of assignment pairs in which one assignment is entirely contained
 * within the other.
 */

fn load_input() -> Vec<(u32, u32, u32, u32)> {
    let mut input = Vec::new();
    // read file "input" line by line
    for line in std::fs::read_to_string("input").unwrap().lines() {
        // split the line into two parts
        let mut parts = line.split(',');
        let mut first = parts.next().unwrap().split('-');
        let mut second = parts.next().unwrap().split('-');
        input.push((
            first.next().unwrap().parse().unwrap(),
            first.next().unwrap().parse().unwrap(),
            second.next().unwrap().parse().unwrap(),
            second.next().unwrap().parse().unwrap(),
        ));
    }
    input
}

fn main() {
    let input = load_input();
    let mut count = 0;
    for (a1, a2, b1, b2) in &input {
        if (a1 >= b1 && a2 <= b2) || (b1 >= a1 && b2 <= a2) {
            count += 1;
        }
    }
    println!("{} pairs overlap entirely", count);

    // now, find the number of pairs that overlap at all
    count = 0;
    for (a1, a2, b1, b2) in &input {
        if (a1 <= b1 && a2 >= b1) || (b1 <= a1 && b2 >= a1) {
            count += 1;
        }
    }
    println!("{} pairs overlap at all", count);
}
