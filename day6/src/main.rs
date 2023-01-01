
// find start-of-packet marker
// which is the 1-based index of the first character in the string
// for which the past 4 characters (inclusive) are all different
fn find_marker(s: &str) -> usize {
    let mut marker = 0;
    for i in 4..s.len() {
        // get the 4 characters before the current character
        let chars = &s[i - 4..i + 1];
        // check if all characters are different
        if chars.chars().all(|c| chars.chars().filter(|&x| x == c).count() == 1) {
            marker = i;
            break;
        }
    }
    marker
}

fn main() {

    // list of example strings and their expected results
    let examples : Vec<(&str, usize)> = vec![
        ("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 7),
        ("bvwbjplbgvbhsrlpgdmjqwftvncz", 5),
        ("nppdvjthqldpwncqszvftbrmjlhg", 6),
        ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 10),
        ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 11),
    ];

    for (s, expected) in &examples {
        let marker = find_marker(s);
        println!("{} -> {}", s, marker);
        if marker != *expected {
            println!("  Expected {} but got {}", expected, marker);
        }
    }

    // now run the real input reading from file "input"
    let input = std::fs::read_to_string("input").unwrap();
    let marker = find_marker(&input);
    println!("{} -> {}", input, marker);

}
