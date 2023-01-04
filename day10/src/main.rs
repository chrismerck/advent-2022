use std::fs::File;
use std::io::{BufRead, BufReader};

// 40x6 screen
type CRT = [[char; 40]; 6];

#[derive(Clone)]
#[derive(Debug)]
enum Instruction {
    AddX(i32),
    Noop,
}

struct Program {
    instructions: Vec<Instruction>,
    pc: usize,
    acc: i32,
    rt: i32,
    total_signal: i32,
    crt: CRT,
}

impl Program {
    fn new() -> Program {
        Program {
            instructions: Vec::new(),
            pc: 0,
            acc: 1,
            rt: 0,
            total_signal: 0,
            crt: [['.'; 40]; 6],
        }
    }

    /*
    read input file

    Example:

        addx 15
        addx -11
        addx 6
        noop
        addx 5
        addx -1
    */
    fn load(&mut self, filename: &str) {
        let file = File::open(filename).expect("Failed to open file");
        let reader = BufReader::new(file);
        for line in reader.lines() {
            let line = line.expect("Failed to read line");
            let mut parts = line.split_whitespace();
            let op = parts.next().expect("Missing opcode");
            // read optional argument
            let arg = parts.next().map(|s| s.parse::<i32>().expect("Failed to parse argument"));
            match op {
                "addx" => {
                    let arg = arg.expect("Missing argument");
                    self.instructions.push(Instruction::AddX(arg));
                }
                "noop" => {
                    self.instructions.push(Instruction::Noop);
                }
                _ => panic!("Unknown opcode"),
            }
        }
    }

    fn advance_runtime(&mut self, n: i32) {
        const MEASUREMENT_TIMES : [i32; 6] = [20, 60, 100, 140, 180, 220];
        for i in 0..n {
            // update crt
            // we draw to the rt'th pixel (0-based), scanning from left to right, top to bottom
            // we draw a pixel if the 3-pixel sprite is visibile from the current horizontal position
            // The sprite is centered on the acc value.
            let horiz = self.rt % 40;
            let vert = self.rt / 40;
            let draw = (horiz - self.acc).abs() < 2;
            if draw {
                self.crt[vert as usize][horiz as usize] = '#';
            }

            // dump state
            /* example:
            Sprite position: ###.....................................

            Start cycle   1: begin executing addx 15
            During cycle  1: CRT draws pixel in position 0
            Current CRT row: #

            During cycle  2: CRT draws pixel in position 1
            Current CRT row: ##
            End of cycle  2: finish executing addx 15 (Register X is now 16)
            Sprite position: ...............###......................

            Start cycle   3: begin executing addx -11
            During cycle  3: CRT draws pixel in position 2
            Current CRT row: ##.

            During cycle  4: CRT draws pixel in position 3
            Current CRT row: ##..
            End of cycle  4: finish executing addx -11 (Register X is now 5)
            Sprite position: ....###.................................

            Start cycle   5: begin executing addx 6
            During cycle  5: CRT draws pixel in position 4
            Current CRT row: ##..#
            */
            let mut sprite = String::new();
            for i in 0..40 {
                if (i as i32 - self.acc).abs() < 2 {
                    sprite.push('#');
                } else {
                    sprite.push('.');
                }
            }
            if self.rt == 1 {
                println!("Sprite position: {}", sprite);
                println!();
            }
            println!();
            if i == 0 {
                println!("Start cycle {:3}: begin executing {:?}", self.rt, self.instructions[self.pc]);
            }
            if draw {
                println!("During cycle {:3}: CRT draws pixel in position {}", self.rt, horiz);
                println!("Current CRT row: {}", self.crt[vert as usize].iter().collect::<String>());
                println!();
            }
            if i == n - 1 {
                println!("End of cycle {:3}: finish executing {:?}", self.rt, self.instructions[self.pc]);
                println!("Sprite position: {}", sprite);
                println!();
            }

            // advance runtime
            self.rt += 1;

            // compute signal strength
            if MEASUREMENT_TIMES.contains(&self.rt) {
                // compute signal strength as rt * acc:
                println!("rt={} acc={} signal={}", self.rt, self.acc, self.rt * self.acc);
                self.total_signal += self.rt * self.acc;
            }
        }
    }

    fn run(&mut self) {
        loop {
            if self.pc >= self.instructions.len() {
                break;
            }
            let instruction = self.instructions[self.pc].clone();
            match instruction {
                Instruction::AddX(arg) => {
                    self.advance_runtime(2);
                    self.acc += arg;
                    self.pc += 1;
                }
                Instruction::Noop => {
                    self.advance_runtime(1);
                    self.pc += 1;
                }
            }
        }
    }

    fn print_crt(&self) {
        for row in self.crt.iter() {
            for c in row.iter() {
                print!("{}", c);
            }
            println!();
        }
    }
}

fn main() {
    let mut program = Program::new();
    program.load("input");
    program.run();
    println!("Total signal: {}", program.total_signal);

    // print CRT
    program.print_crt();
}
