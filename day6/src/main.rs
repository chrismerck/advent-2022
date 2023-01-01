
// find start-of-packet marker
// which is the 1-based index of the first character in the string
// for which the past n characters (inclusive) are all different
// add a parameter for the number of characters to check
fn find_marker(s: &str, n: usize) -> usize {
    let mut marker = 0;
    for i in n..s.len() {
        // get the n characters before the current character
        let chars = &s[i - n..i];
        //println!("   {} -> {}", i, chars);
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
        let marker_4 = find_marker(s, 4);
        let marker_14 = find_marker(s, 14);
        println!("{} -> {} {}", s, marker_4, marker_14);
    }

    // now run the real input reading from file "input"
    let input = std::fs::read_to_string("input").unwrap();
    let marker_4 = find_marker(&input, 4);
    let marker_14 = find_marker(&input, 14);
    println!("{} -> {} {}", input, marker_4, marker_14);
}
