use std::collections::VecDeque;

const ROUNDS_P1: u32 = 20;
const ROUNDS_P2: u32 = 10000;

#[derive(Clone)]
struct Monkey {
    items: VecDeque<u64>,
    operation: fn(u64) -> u64,
    divisor: u64,
    next_monkeys: (usize, usize),
}

pub fn day_eleven(_input: String) {
    // Hardcoded monkeys to start with for speed
    // I will parse them properly later
    let monkeys = vec![
        Monkey {
          items: VecDeque::from([85, 79, 63, 72]),
          operation: |old| old * 17,
          divisor: 2,
            next_monkeys: (2, 6)
        },

        Monkey {
          items: VecDeque::from([53, 94, 65, 81, 93, 73, 57, 92]),
          operation: |old| old * old,
          divisor: 7,
            next_monkeys: (0, 2)
        },

        Monkey {
          items: VecDeque::from([62, 63]),
          operation: |old| old + 7,
          divisor: 13,
            next_monkeys: (7, 6)
        },

        Monkey {
          items: VecDeque::from([57, 92, 56]),
          operation: |old| old + 4,
          divisor: 5,
            next_monkeys: (4, 5)
        },

        Monkey {
          items: VecDeque::from([67]),
          operation: |old| old + 5,
          divisor: 3,
            next_monkeys: (1, 5)
        },

        Monkey {
          items: VecDeque::from([85, 56, 66, 72, 57, 99]),
          operation: |old| old + 6,
          divisor: 19,
            next_monkeys: (1, 0)
        },

        Monkey {
          items: VecDeque::from([86, 65, 98, 97, 69]),
          operation: |old| old * 13,
          divisor: 11,
            next_monkeys: (3, 7)
        },

        Monkey {
          items: VecDeque::from([87, 68, 92, 66, 91, 50, 68]),
          operation: |old| old + 2,
          divisor: 17,
            next_monkeys: (4, 3)
        },
    ];

    part_one(monkeys.clone());
    part_two(monkeys);
}

fn part_one(mut monkeys: Vec<Monkey>) {
    // Part 1
    let num_monkeys = monkeys.len();
    let mut inspections = vec![0; num_monkeys];

    for _ in 0..ROUNDS_P1 {
        for i in 0..num_monkeys {
            while let Some(item) = monkeys[i].items.pop_front() {
                inspections[i] += 1;

                let new = (monkeys[i].operation)(item);
                let new = ((new as f64) / 3.0).floor() as u64;

                let next_monkey = if new % monkeys[i].divisor == 0 {
                    monkeys[i].next_monkeys.0
                } else {
                    monkeys[i].next_monkeys.1 
                };

                monkeys[next_monkey].items.push_back(new);
            }
        }
    }

    inspections.sort();
    println!("{}", inspections[num_monkeys - 1] * inspections[num_monkeys - 2]);
}

fn part_two(mut monkeys: Vec<Monkey>) {
    let num_monkeys = monkeys.len();
    let modulus: u64 = monkeys.iter().map(|monkey| monkey.divisor).product();

    let mut inspections = vec![0u64; num_monkeys];

    for _ in 0..ROUNDS_P2 {
        for i in 0..num_monkeys {
            while let Some(item) = monkeys[i].items.pop_front() {
                inspections[i] += 1;

                let new = (monkeys[i].operation)(item);
                let new = new % modulus;

                let next_monkey = if new % monkeys[i].divisor == 0 {
                    monkeys[i].next_monkeys.0
                } else {
                    monkeys[i].next_monkeys.1 
                };

                monkeys[next_monkey].items.push_back(new);
            }
        }
    }

    inspections.sort();
    println!("{}", inspections[num_monkeys - 1] * inspections[num_monkeys - 2]);
}
