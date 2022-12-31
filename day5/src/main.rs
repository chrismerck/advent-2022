
/*

Input example:

    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2

*/

use std::fs::File;
use std::io::{BufRead, BufReader};

// read from input file,
// outpuiting a vector of vectors of chars representing each stack of crates,
// with the top crate at the end of the vector,
// and a vector of moves to be made, each move being a tuple of the form 
// (from_stack, to_stack, quantity)
fn read_input() -> (Vec<Vec<char>>, Vec<(usize, usize, usize)>) {
    let mut file = BufReader::new(File::open("input").unwrap());
    (read_input_stacks(&mut file), read_input_moves(&mut file))
}

// read the stacks of crates from the input file
fn read_input_stacks(file: &mut BufReader<File>) -> Vec<Vec<char>> {
    let mut stacks = Vec::new();
    // read line by line into a string
    for line in file.lines() {
        let line = line.unwrap();
        // if the line is empty, skip it
        if line.trim().is_empty() {
           continue;
        }
        // if 2nd character in line is 1, we have reached the end of the stacks
        if line.chars().nth(1).unwrap() == '1' {
            break;
        }
        // the line contains a slice of stacks in the form:
        // r"((   |\[\w\]) )+"
        // where each column is either empty or contains a crate
        // and each crate contains a capital letter.
        // We can simply seek to the (i - 1)*4 + 1th character in the line
        // and check if it contains a capital letter.
        let line_len = line.len();
        let stacks_this_slice = (line_len + 1) / 4;
        // expand stacks if necessary
        while stacks.len() < stacks_this_slice {
            stacks.push(Vec::new());
        }
        for i in 1..stacks_this_slice + 1 {
            let crate_char = line.chars().nth((i - 1) * 4 + 1).unwrap();
            if crate_char.is_uppercase() {
                stacks[i - 1].insert(0, crate_char);
            }
        }
    }
    stacks
}

// read the moves from the input file
fn read_input_moves(file: &mut BufReader<File>) -> Vec<(usize, usize, usize)> {
    let mut moves = Vec::new();
    // read line by line into a string
    for line in file.lines() {
        let line = line.unwrap();
        // if the line is empty, skip it
        if line.trim().is_empty() {
           continue;
        }
        // the line contains a move in the form:
        // r"move \d+ from \d+ to \d+"
        // where the first number is the quantity of crates to move,
        // the second is the stack to move from,
        // and the third is the stack to move to.
        let mut words = line.split_whitespace();
        let quantity = words.nth(1).unwrap().parse::<usize>().unwrap();
        let from_stack = words.nth(1).unwrap().parse::<usize>().unwrap();
        let to_stack = words.nth(1).unwrap().parse::<usize>().unwrap();
        moves.push((from_stack, to_stack, quantity));
    }
    moves
}

// move crates from one stack to another
fn move_part1(stacks: &mut Vec<Vec<char>>, moves: &Vec<(usize, usize, usize)>) {
    for (from_stack, to_stack, quantity) in moves {
        // pop one crate at a time from the from_stack,
        // and push it onto the to_stack
        for _ in 0..*quantity {
            let crate_char = stacks[*from_stack - 1].pop().unwrap();
            stacks[*to_stack - 1].push(crate_char);
        }
    }
}

// move crates, this time maintaining the order of the crates
// in each stack as they are moved
fn move_part2(stacks: &mut Vec<Vec<char>>, moves: &Vec<(usize, usize, usize)>) {
    for (from_stack, to_stack, quantity) in moves {
        // pop the crates from the from_stack,
        // and push them onto a temporary stack
        let mut temp_stack = Vec::new();
        for _ in 0..*quantity {
            let crate_char = stacks[*from_stack - 1].pop().unwrap();
            temp_stack.push(crate_char);
        }
        // pop the crates from the temporary stack,
        // and push them onto the to_stack
        for _ in 0..*quantity {
            let crate_char = temp_stack.pop().unwrap();
            stacks[*to_stack - 1].push(crate_char);
        }
    }
}

// accept an argument specifying --part1 or --part2
fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        panic!("Usage: cargo run --release --bin day5 -- --[part1|part2]");
    }

    let (mut stacks, moves) = read_input();

    // move crates
    match args[1].as_str() {
        "--part1" => move_part1(&mut stacks, &moves),
        "--part2" => move_part2(&mut stacks, &moves),
        _ => panic!("Usage: cargo run --release --bin day5 -- --[part1|part2]"),
    }

    // print final stacks
    for (i, stack) in stacks.iter().enumerate() {
        println!("Stack {}: {:?}", i + 1, stack);
    }
    // print moves
    for (from_stack, to_stack, quantity) in moves {
        println!("Move {} from {} to {}", quantity, from_stack, to_stack);
    }
    // construct and print a string with the top crate of each stack
    let mut top_crates = String::new();
    for stack in &stacks {
        top_crates.push(stack[stack.len() - 1]);
    }
    println!("Top crates: {}", top_crates);
}
