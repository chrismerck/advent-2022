
// read an input file containing lines of the form:
//  <their-move> <my-move>
// where <their-move> and <my-move> are capital letters.
// returns a vector of tuples of the form:
//  (their-move, my-move)
fn read_input() -> Vec<(char, char)> {
    std::fs::read_to_string("input")
        .unwrap()
        .lines()
        .map(|line| {
            let mut chars = line.chars();
            let their_move = chars.next().unwrap();
            let my_move = chars.skip(1).next().unwrap();
            (their_move, my_move)
        })
        .collect()
}

// convert moves using interpretation from part 1
fn convert_moves_part1(moves: &[(char, char)]) -> Vec<(char, char)> {
    moves
        .iter()
        .map(|(their_move, my_move)| (convert_move(*their_move), convert_move(*my_move)))
        .collect()
}

// convert A, B, C, to R, P, S respectively
// and also X, Y, Z, to R, P, S
// but R, P, S are unchanged
fn convert_move(c: char) -> char {
    match c {
        'A' => 'R',
        'B' => 'P',
        'C' => 'S',
        'X' => 'R',
        'Y' => 'P',
        'Z' => 'S',
        'R' => 'R',
        'P' => 'P',
        'S' => 'S',
        _ => panic!("invalid move"),
    }
}

// compute the score of a game of rock-paper-scissors
// given the moves of the two players
// returns a tuple of the form:
//  (my-score, their-score)
// scores are 6 for a win, 3 for a draw (tie), 0 for a loss
fn base_score((their_move, my_move): (char, char)) -> (isize, isize) {
    match (their_move, my_move) {
        ('R', 'R') => (3, 3),
        ('R', 'P') => (6, 0),
        ('R', 'S') => (0, 6),
        ('P', 'R') => (0, 6),
        ('P', 'P') => (3, 3),
        ('P', 'S') => (6, 0),
        ('S', 'R') => (6, 0),
        ('S', 'P') => (0, 6),
        ('S', 'S') => (3, 3),
        _ => panic!("invalid move"),
    }
}

// compute the bonus score based on which move was made
// 1, 2, 3 points for R, P, S respectively
fn bonus_score(move_char: char) -> isize {
    match move_char {
        'R' => 1,
        'P' => 2,
        'S' => 3,
        _ => panic!("invalid move"),
    }
}

// compute round score given the moves of the two players
// returns a tuple of the form:
//  (my-score, their-score)
fn score(move_pair: (char, char)) -> (isize, isize) {
    let (my_score, their_score) = base_score(move_pair);
    let (their_move, my_move) = move_pair;
    (
        my_score + bonus_score(my_move),
        their_score + bonus_score(their_move),
    )
}


// find the total score for each player given a vector of moves
// returns a tuple of the form:
//  (my-score, their-score)
fn total_score(moves: &[(char, char)]) -> (isize, isize) {
    moves
        .iter()
        .map(|move_pair| score(*move_pair))
        .fold((0, 0), |(my_score, their_score), (my_score2, their_score2)| {
            (my_score + my_score2, their_score + their_score2)
        })
}

// PART 2 SPECIFIC CODE

// convert column 1 according to interpretation from part 2
// X -> L (lose), Y -> D (draw), Z -> W (win)
// also supports converting from R, P, S to L, D, W
fn convert_move_part2(c: char) -> char {
    match c {
        'X' => 'L',
        'Y' => 'D',
        'Z' => 'W',
        'R' => 'L',
        'P' => 'D',
        'S' => 'W',
        _ => panic!("invalid move"),
    }
}

fn convert_moves_part2(moves: &[(char, char)]) -> Vec<(char, char)> {
    moves
        .iter()
        .map(|(their_move, my_move)| (convert_move(*their_move), convert_move_part2(*my_move)))
        .collect()
}

// compute move required for us to lose, draw, or win
// given (their-move, outcome)
// returns a char representing the move required on my part
// Example: (R, L) -> S  (they played R, we need to play S to lose)
fn my_required_move((their_move, outcome): (char, char)) -> char {
    match (their_move, outcome) {
        ('R', 'L') => 'S',
        ('R', 'D') => 'R',
        ('R', 'W') => 'P',
        ('P', 'L') => 'R',
        ('P', 'D') => 'P',
        ('P', 'W') => 'S',
        ('S', 'L') => 'P',
        ('S', 'D') => 'S',
        ('S', 'W') => 'R',
        _ => panic!("invalid move"),
    }
}

// compute the move pair required to achieve the desired outcome
// given (their-move, outcome)
// returns a tuple of the form:
//  (their-move, my-move)
fn required_move_pair((their_move, outcome): (char, char)) -> (char, char) {
    (their_move, my_required_move((their_move, outcome)))
}

fn main() {
    // read the input file
    let moves = read_input();
    let moves = convert_moves_part1(&moves);
    println!("Part 1:");
    let (my_score, their_score) = total_score(&moves);
    println!("{} {}", my_score, their_score);

    println!("Part 2:");
    let their_moves_and_outcomes = convert_moves_part2(&moves);
    let moves = their_moves_and_outcomes
        .iter()
        .map(|(their_move, outcome)| required_move_pair((*their_move, *outcome)))
        .collect::<Vec<(char, char)>>();
    let (my_score, their_score) = total_score(&moves);
    println!("{} {}", my_score, their_score);
}
