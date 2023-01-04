use std::fs::File;
use std::io::{BufRead, BufReader};


// clonable
#[derive(Clone)]
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
}

impl Program {
    fn new() -> Program {
        Program {
            instructions: Vec::new(),
            pc: 0,
            acc: 1,
            rt: 0,
            total_signal: 0,
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
        for _ in 0..n {
            self.rt += 1;
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
}

fn main() {
    let mut program = Program::new();
    program.load("input");
    program.run();
    println!("Total signal: {}", program.total_signal);
}
