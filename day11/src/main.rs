
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
enum Operation {
    Add(i32),
    Multiply(i32),
    Square,
}

struct Monkey {
    id: usize,
    items: Option<Vec<i32>>,
    operation: Option<Operation>,
    test_divisor: i32,
    // if dividible, throw_to[0], else throw_to[1]
    throw_to: (usize, usize),
}

impl Monkey {
    fn new(id : usize, items: Option<Vec<i32>>, operation: Option<Operation>, test_divisor: i32, throw_to: (usize, usize)) -> Monkey {
        Monkey {
            id,
            items,
            operation,
            test_divisor,
            throw_to,
        }
    }

    fn set_items(&mut self, items: Vec<i32>) {
        self.items = Some(items);
    }

    fn set_operation(&mut self, operation: Operation) {
        self.operation = Some(operation);
    }

    fn set_test_divisor(&mut self, test_divisor: i32) {
        self.test_divisor = test_divisor;
    }

    fn set_throw_to(&mut self, branch : bool, throw_to: usize) {
        if branch {
            self.throw_to.0 = throw_to;
        } else {
            self.throw_to.1 = throw_to;
        }
    }
}

struct MonkeyBusiness {
    monkeys: Vec<Monkey>,
}

impl MonkeyBusiness {
    fn new() -> MonkeyBusiness {
        MonkeyBusiness {
            monkeys: Vec::new(),
        }
    }

    fn add_monkey(&mut self, id: usize) {
        self.monkeys.push(Monkey::new(id, None, None, 0, (0, 0)));
        assert_eq!(self.monkeys.len(), id);
    }

    /*
    Example input:

    Monkey 0:
    Starting items: 79, 98
    Operation: new = old * 19
    Test: divisible by 23
        If true: throw to monkey 2
        If false: throw to monkey 3

    Monkey 1:
    Starting items: 54, 65, 75, 74
    Operation: new = old + 6
    Test: divisible by 19
        If true: throw to monkey 2
        If false: throw to monkey 0

    Monkey 2:
    Starting items: 79, 60, 97
    Operation: new = old * old
    Test: divisible by 13
        If true: throw to monkey 1
        If false: throw to monkey 3

    Monkey 3:
    Starting items: 74
    Operation: new = old + 3
    Test: divisible by 17
        If true: throw to monkey 0
        If false: throw to monkey 1
    */

    fn load(&mut self, filename: &str) {
        // parse input file step-by-step
        // creating Monkeys, adding them to MonkeyBusiness,
        // and then (on a second pass) setting the throw_to fields to point to the correct Monkey
        let file = File::open(filename).expect("Failed to open file");
        let reader = BufReader::new(file);
        let mut current_monkey : Option<&mut Monkey> = None;
        // on first pass, create Monkeys and add them to MonkeyBusiness
        for line in reader.lines() {
            // get first word
            let line = line.expect("Failed to read line");
            let mut parts = line.split_whitespace();
            let word = parts.next().expect("Missing word");
            match word {
                "Monkey" => {
                    // create new Monkey
                    let monkey_id = parts.next().expect("Missing monkey id");
                    let monkey_id = monkey_id.parse::<usize>().expect("Failed to parse monkey id");
                    // create new Monkey
                    self.add_monkey(monkey_id);
                    // set current_monkey
                    current_monkey = Some(&mut self.monkeys[monkey_id - 1]);
                },
                "Starting" => {
                    // get items, skipping "items:"
                    let items = parts.skip(1).collect::<Vec<&str>>();
                    // parse items
                    let items = items.iter().map(|item| item.parse::<i32>().expect("Failed to parse item")).collect::<Vec<i32>>();
                    // set items
                    current_monkey.as_mut().unwrap().set_items(items);
                },
                "Operation:" => {
                    // skip "new = ", asserting that it is there
                    assert_eq!(parts.next().expect("Missing 'new = '"), "new");
                    assert_eq!(parts.next().expect("Missing 'new = '"), "=");
                    // parse left-operand, operator, right-operand
                    let left_operand = parts.next().expect("Missing left operand");
                    let operator = parts.next().expect("Missing operator");
                    let right_operand = parts.next().expect("Missing right operand");
                    // assert left operand is "old"
                    assert_eq!(left_operand, "old");
                    // parse right operand (either number or "old")
                    let right_operand = match right_operand {
                        "old" => None,
                        _ => Some(right_operand.parse::<i32>().expect("Failed to parse right operand")),
                    };
                    // parse operator (either "*" or "+"), and create Operation
                    // considering case where right_operand is None and operator is "*", which is the Square operation
                    let operation = match operator {
                        "*" => match right_operand {
                            Some(n) => Operation::Multiply(n),
                            None => Operation::Square,
                        },
                        "+" => Operation::Add(right_operand.unwrap()),
                        _ => panic!("Unknown operator"),
                    };
                    // set operation
                    current_monkey.as_mut().unwrap().set_operation(operation);
                },
                "Test:" => {
                    // skip "divisible by ", asserting that it is there
                    assert_eq!(parts.next().expect("Missing 'divisible by '"), "divisible");
                    assert_eq!(parts.next().expect("Missing 'divisible by '"), "by");
                    // parse test divisor
                    let test_divisor = parts.next().expect("Missing test divisor");
                    let test_divisor = test_divisor.parse::<i32>().expect("Failed to parse test divisor");
                    assert!(test_divisor > 0);
                    // set test divisor
                    current_monkey.as_mut().unwrap().set_test_divisor(test_divisor);
                },
                "If" => {
                    // for now we skip the throw_to fields, we'll set them later in 2nd pass
                },
                _ => panic!("Unknown word '{}'", word),
            }
        }
        // on second pass, set throw_to fields
        let file = File::open(filename).expect("Failed to open file");
        let reader = BufReader::new(file);
        let mut current_monkey : Option<&mut Monkey> = None;
        for line in reader.lines() {
            // get first word
            let line = line.expect("Failed to read line");
            let mut parts = line.split_whitespace();
            let word = parts.next().expect("Missing word");
            match word {
                "Monkey" => {
                    // get monkey id
                    let monkey_id = parts.next().expect("Missing monkey id");
                    let monkey_id = monkey_id.parse::<usize>().expect("Failed to parse monkey id");
                    // set current_monkey
                    current_monkey = Some(&mut self.monkeys[monkey_id - 1]);
                },
                "If" => {
                    // skip "true:" or "false:", asserting that it is there
                    let true_or_false = parts.next().expect("Missing 'true:' or 'false:'");
                    assert!(true_or_false == "true:" || true_or_false == "false:");
                    // parse monkey id to throw to
                    let monkey_id = parts.next().expect("Missing monkey id");
                    let monkey_id = monkey_id.parse::<usize>().expect("Failed to parse monkey id");
                    // set throw_to id
                    current_monkey.as_mut().unwrap().set_throw_to(true_or_false == "true:", monkey_id);
                },
                _ => {},
            }
        }
    }

    fn run(&mut self, num_turns: i32) {
        // iterate over monkeys, each monkey gets a turn
        for turn in 0..num_turns {
            println!("=== Turn {} ===", turn + 1);
            self.print_items();
            for i in 0..self.monkeys.len() {
                // process all items
                let monkey = &mut self.monkeys[i];
                while let Some(item) = monkey.items.as_mut().unwrap().pop() {
                    // apply operation
                    let new_item = match monkey.operation.as_mut().unwrap() {
                        Operation::Add(n) => item + *n,
                        Operation::Multiply(n) => item * *n,
                        Operation::Square => item * item,
                    };
                    // test
                    let throw_to = if new_item % monkey.test_divisor == 0 {
                        monkey.throw_to.0
                    } else {
                        monkey.throw_to.1
                    };
                    // throw by pushing to back of queue for monkey id "throw_to"
                    self.monkeys[throw_to - 1].items.as_mut().unwrap().push(new_item);
                }
            }
        }
    }

    fn print_rules(&self) {
        for id in 0..self.monkeys.len() {
            let monkey = &self.monkeys[id];
            println!("Monkey {}:", id + 1);
            println!("  Items: {:?}", monkey.items);
            println!("  Operation: {:?}", monkey.operation);
            println!("  Test: {}", monkey.test_divisor);
            println!("  Throw to: {} {}", monkey.throw_to.0, monkey.throw_to.1);
        }
    }

    fn print_items(&self) {
        for id in 0..self.monkeys.len() {
            let monkey = &self.monkeys[id];
            println!("Monkey {}: {:?}", id + 1, monkey.items);
        }
    }
}

fn main() {
    let mut monkey_business = MonkeyBusiness::new();
    monkey_business.load("input.example");
    monkey_business.print_rules();
    monkey_business.run(20);
}


