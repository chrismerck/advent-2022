
// read list of list of integers from file
// The lists are seperated by a blank line.
// The integers in each list are seperated by newline.
// Returns a vector of vectors of integers.
fn read_input() -> Vec<Vec<isize>> {
    let mut input = Vec::new();
    let mut current_list = Vec::new();
    for line in std::fs::read_to_string("input").unwrap().lines() {
        if line.is_empty() {
            input.push(current_list);
            current_list = Vec::new();
        } else {
            current_list.push(line.parse().unwrap());
        }
    }
    input.push(current_list);
    input
}

// compute the sum of a list of integers
fn sum(list: &[isize]) -> isize {
    list.iter().sum()
}

// find argmax of the sums of the lists
// That is, this is the index of the elf with the most calories.
// with 1-based indexing
fn argmax_sums(elves: &[Vec<isize>]) -> usize {
    elves
        .iter()
        .enumerate()
        .map(|(i, list)| (i, sum(list)))
        .max_by_key(|(_, sum)| *sum)
        .unwrap()
        .0
        + 1
}

// find the three elves with the most calories
// This requires first creating a vector with tuples of original index and sum:
//  [ (1, sum1), (2, sum2), ... ]
// Then sorting this vector by sum in descending order.
// Then taking the first three elements of the sorted vector.
// And returning the original indices of these elements.
// the indecies shall be 1-based.
fn argmax_sums_3(elves: &[Vec<isize>]) -> Vec<usize> {
    let mut sums = elves
        .iter()
        .enumerate()
        .map(|(i, list)| (i, sum(list)))
        .collect::<Vec<(usize, isize)>>();
    sums.sort_by_key(|(_, sum)| -sum);
    sums.iter().take(3).map(|(i, _)| i + 1).collect()
}

fn main() {
    let elves = read_input();
    // find the elf carrying the most calories
    let argmax = argmax_sums(&elves);
    println!("# PART 1");
    println!("The elf carrying the most calories is {}", argmax);
    println!("  and he has {} calories.", sum(&elves[argmax - 1]));

    println!("\n# PART 2");
    // find the three elves carrying the most calories
    let argmaxes = argmax_sums_3(&elves);
    println!(
        "The elves carrying the most calories are {}",
        argmaxes
            .iter()
            .map(|i| i.to_string())
            .collect::<Vec<String>>()
            .join(", ")
    );
    println!(
        "  and those three elves have {} calories.",
        argmaxes.iter().map(|i| sum(&elves[i - 1])).sum::<isize>()
    );

}
