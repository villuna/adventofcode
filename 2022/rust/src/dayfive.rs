use itertools::Itertools;

type Stack = Vec<char>;
type Instruction = (usize, usize, usize);

#[derive(Clone)]
struct CargoShip {
    stacks: Vec<Stack>,
    instructions: Vec<Instruction>,
}

impl CargoShip {
    // Runs the simulation
    fn run(&mut self) {
        for &(num, from, to) in self.instructions.iter() {
            for _ in 0..num {
                let item = self.stacks[from - 1].pop().unwrap();

                self.stacks[to - 1].push(item);
            }
        }
    }

    fn run_9001(&mut self) {
        for &(num, from, to) in self.instructions.iter() {
            let mut temp = Vec::new();

            for _ in 0..num {
                let item = self.stacks[from - 1].pop().unwrap();
                temp.push(item);
            }

            for _ in 0..num {
                let item = temp.pop().unwrap();
                self.stacks[to - 1].push(item);
            }
        }
    }

    // Gets a string of all the crates at the top of each stack
    fn top(&self) -> String {
        let mut res = String::new();

        for stack in self.stacks.iter() {
            res.push(*stack.last().unwrap());
        }

        return res;
    }
}

fn parse_stacks(stack_input: &str) -> Vec<Stack> {
    let mut stacks = Vec::new();
    let mut lines = stack_input.lines().collect_vec();

    // The last line is just 1 2 3 4 5 etc
    lines.pop().unwrap();

    for _ in 0..9 {
        stacks.push(Vec::new());
    }

    for line in lines {
        // The character for the nth stack will be at index 4n + 1, where n
        // is the nth stack (starting at 0)
        let mut chars = line.chars();
        let mut i = 0;
        chars.next().unwrap();

        while let Some(c) = chars.next() {
            if !c.is_whitespace() {
                stacks[i].insert(0, c);
            }

            // Go to the next stack and advance the iterator by 3 more chars
            i += 1;
            chars.nth(2);
        }
    }

    println!("{stacks:?}");

    return stacks;
}

fn parse_instructions(instr_input: &str) -> Vec<Instruction> {
    instr_input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            // Have to do this for the compiler to understand the type
            // because collect_tuple is a fucked up function bruh
            let res: Instruction = line
                .split(" ")
                .filter_map(|s| s.parse::<usize>().ok())
                .collect_tuple()
                .unwrap();

            res
        })
        .collect_vec()
}

fn parse_input(input: &str) -> CargoShip {
    // Split at the empty line
    let (head, tail) = input
        .split("\n\n")
        .collect_tuple()
        .expect("Malformed input");

    CargoShip {
        stacks: parse_stacks(head),
        instructions: parse_instructions(tail),
    }
}

pub fn day_five(input: String) {
    let cargo = parse_input(&input);

    part_one(cargo.clone());
    part_two(cargo);
}

fn part_one(mut cargo: CargoShip) {
    // This is the funniest rust joke of the century
    // you may now laugh
    cargo.run();

    println!("{}", cargo.top());
}

fn part_two(mut cargo: CargoShip) {
    cargo.run_9001();

    println!("{}", cargo.top());
}
