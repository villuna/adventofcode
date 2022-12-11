use itertools::Itertools;
use regex::Regex;
use std::collections::VecDeque;

// The reason i moved to a vec of structs isn't for any high minded
// data oriented reason or whatever
// it's because Box<dyn Fn(u64) -> u64> doesn't implement Clone
struct Monkeys {
    items: Vec<VecDeque<u64>>,
    operations: Vec<Box<dyn Fn(u64) -> u64>>,
    divisors: Vec<u64>,
    next_monkeys: Vec<(usize, usize)>,
}

fn parse_monkeys(input: &str) -> Monkeys {
    let re: Regex = Regex::new(
        r"Monkey \d+:\n\s*Starting items: ((?:\d+)(?:, \d+)*)
\s*Operation: new = old (\*|\+) (\d+|old)
\s*Test: divisible by (\d+)
\s*If true: throw to monkey (\d+)
\s*If false: throw to monkey (\d+)",
    )
    .unwrap();

    let mut monkeys = Monkeys {
        items: vec![],
        operations: vec![],
        divisors: vec![],
        next_monkeys: vec![],
    };

    for captures in re.captures_iter(input) {
        let (items, operator, operand, divisor, true_monkey, false_monkey) = captures
            .iter()
            .skip(1)
            .map(|m| m.unwrap().as_str().to_string())
            .collect_tuple()
            .unwrap();

        monkeys.items.push(VecDeque::from_iter(
            items.split(", ").map(|num| num.parse::<_>().unwrap()),
        ));
        monkeys.operations.push(Box::new(move |old| {
            let operand = if operand == "old" {
                old
            } else {
                operand.parse().unwrap()
            };

            if operator == "*" {
                old * operand
            } else {
                old + operand
            }
        }));

        monkeys.divisors.push(divisor.parse().unwrap());
        monkeys
            .next_monkeys
            .push((true_monkey.parse().unwrap(), false_monkey.parse().unwrap()));
    }

    monkeys
}

pub fn day_eleven(input: String) {
    let mut monkeys = parse_monkeys(&input);
    let items = monkeys.items.clone();

    // Part one
    simulate_monkeys(&mut monkeys, 20, true);

    // reset the items
    monkeys.items = items;
    simulate_monkeys(&mut monkeys, 10000, false);
}

fn simulate_monkeys(monkeys: &mut Monkeys, rounds: u32, dividing: bool) {
    let num_monkeys = monkeys.items.len();
    let modulus: u64 = monkeys.divisors.iter().product();

    let mut inspections = vec![0u64; num_monkeys];

    for _ in 0..rounds {
        for i in 0..num_monkeys {
            while let Some(item) = monkeys.items[i].pop_front() {
                inspections[i] += 1;

                let mut new = (monkeys.operations[i])(item);

                if dividing {
                    new = ((new as f64) / 3.0).floor() as u64;
                }

                new = new % modulus;

                let next_monkey = if new % monkeys.divisors[i] == 0 {
                    monkeys.next_monkeys[i].0
                } else {
                    monkeys.next_monkeys[i].1
                };

                monkeys.items[next_monkey].push_back(new);
            }
        }
    }

    inspections.sort();
    println!(
        "{}",
        inspections[num_monkeys - 1] * inspections[num_monkeys - 2]
    );
}
