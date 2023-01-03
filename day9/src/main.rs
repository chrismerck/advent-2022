use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashSet;

struct Game {
    knots: Vec<(i32, i32)>,
    /// squares visited by last knot
    visited: HashSet<(i32, i32)>,
}

const num_knots: usize = 10;

impl Game {
    fn new() -> Game {
        Game {
            knots: vec![(0, 0); num_knots],
            visited: HashSet::new(),
        }
    }

    fn move_head(&mut self, direction: char, distance: i32) {
        let head = &mut self.knots[0];
        match direction {
            'R' => head.0 += distance,
            'U' => head.1 += distance,
            'L' => head.0 -= distance,
            'D' => head.1 -= distance,
            _ => panic!("Invalid direction: {}", direction),
        }
    }

    fn follow(&mut self, k: usize) {
        // we only need to consider 3 cases:
        // 1. head is two steps away from tail in a cardinal direction.
        //    Tail moves one step to catch up:
        //     H.T --> HT
        // 2. head is two steps away in one direction and one step away in the other direction.
        //    Tail moves one step diagonally to catch up:
        //     H.. --> HT.
        //     ..T     ...
        // 3. head and tail are touching or overlapping
        //    Tail stays in place.
        // 4. head and tail are two steps away diagonally
        //    Tail moves one step diagonally to catch up:
        //     H..     H..
        //     ... --> .T.
        //     ..T     ...
        // All other cases are impossible and should panic.

        let head = self.knots[k - 1];
        let tail = &mut self.knots[k];

        println!("follow: k={} head={:?} tail={:?}", k, head, tail);

        // case 1
        if (head.0 - tail.0).abs() == 2 && head.1 == tail.1 {
            tail.0 += (head.0 - tail.0) / 2;
        } else if (head.1 - tail.1).abs() == 2 && head.0 == tail.0 {
            tail.1 += (head.1 - tail.1) / 2;
        // case 2
        } else if (head.0 - tail.0).abs() == 2 && (head.1 - tail.1).abs() == 1 {
            tail.0 += (head.0 - tail.0) / 2;
            tail.1 += (head.1 - tail.1) / 1;
        } else if (head.1 - tail.1).abs() == 2 && (head.0 - tail.0).abs() == 1 {
            tail.0 += (head.0 - tail.0) / 1;
            tail.1 += (head.1 - tail.1) / 2;
        // case 4
        } else if (head.0 - tail.0).abs() == 2 && (head.1 - tail.1).abs() == 2 {
            tail.0 += (head.0 - tail.0) / 2;
            tail.1 += (head.1 - tail.1) / 2;
        // case 3
        } else {
            // head and tail are touching or in the same row or column
            // tail stays in place
            assert!((head.0 - tail.0).abs() <= 1);
            assert!((head.1 - tail.1).abs() <= 1);
        }

        println!("  follow result: tail={:?}", tail);
    }

    fn visit(&mut self) {
        let tail = self.knots[num_knots - 1];
        self.visited.insert(tail);
    }

    fn run(&mut self, moves: Vec<(char, i32)>) {
        for (direction, distance) in moves {
            // print move
            println!("== {} {} ==", direction, distance);
            println!();
            for _ in 0..distance {
                self.move_head(direction, 1);
                for k in 1..num_knots {
                    self.follow(k);
                    //self.print();
                }
                self.visit();
                //self.print();
                println!();
            }
        }
    }
    
    /* print game state */
    fn print(&self) {
        let mut min_x = 0;
        let mut max_x = 0;
        let mut min_y = 0;
        let mut max_y = 0;
        // This code determines the smallest and largest x and y values
        // including visited and knot squares so that we can print the grid
        for (x, y) in &self.visited {
            if *x < min_x {
                min_x = *x;
            }
            if *x > max_x {
                max_x = *x;
            }
            if *y < min_y {
                min_y = *y;
            }
            if *y > max_y {
                max_y = *y;
            }
        }
        for (x, y) in &self.knots {
            if *x < min_x {
                min_x = *x;
            }
            if *x > max_x {
                max_x = *x;
            }
            if *y < min_y {
                min_y = *y;
            }
            if *y > max_y {
                max_y = *y;
            }
        }
        // print the grid with origin, all knots (H, 1, .. N), and visited squares
        for y in min_y..=max_y {
            for x in min_x..=max_x {
                if self.knots.contains(&(x, y)) {
                    let k = self.knots.iter().position(|&k| k == (x, y)).unwrap();
                    // print knot 0 as 'H' and others as digit
                    if k == 0 {
                        print!("H");
                    } else {
                        print!("{}", k);
                    }
                } else if self.visited.contains(&(x, y)) {
                    print!("#");
                } else if x == 0 && y == 0 {
                    print!("s");
                } else {
                    print!(".");
                }
            }
            println!();
        }

    }

    /* print visited squares, with origin indicated by 's',
      and visited squares indicated by '#'

      Example:
        ..##..
        ...##.
        .####.
        ....#.
        s###..
    */
    fn print_visited(&self) {
        let mut min_x = 0;
        let mut max_x = 0;
        let mut min_y = 0;
        let mut max_y = 0;
        // This code determines the smallest and largest x and y values
        // in the set of visited points so that we can print the grid
        for (x, y) in &self.visited {
            if *x < min_x {
                min_x = *x;
            }
            if *x > max_x {
                max_x = *x;
            }
            if *y < min_y {
                min_y = *y;
            }
            if *y > max_y {
                max_y = *y;
            }
        }
        // print the grid with origin and visited squares
        for y in min_y..=max_y {
            for x in min_x..=max_x {
                if x == 0 && y == 0 {
                    print!("s");
                } else if self.visited.contains(&(x, y)) {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!();
        }
    }
}

fn read_moves() -> Vec<(char, i32)> {
    let file = File::open("input").unwrap();
    let reader = BufReader::new(file);
    let mut moves = Vec::new();
    for line in reader.lines() {
        let line = line.unwrap();
        // example line: R 4
        let mut parts = line.split_whitespace();
        let direction = parts.next().unwrap().chars().next().unwrap();
        let distance = parts.next().unwrap().parse::<i32>().unwrap();
        moves.push((direction, distance));
    }
    moves
}

fn print_moves(moves: &Vec<(char, i32)>) {
    for (direction, distance) in moves {
        println!("{} {}", direction, distance);
    }
}

fn main() {
    let moves = read_moves();
    print_moves(&moves);
    let mut game = Game::new();
    game.run(moves);
    game.print_visited();
    println!("Visited {} squares", game.visited.len());
}
