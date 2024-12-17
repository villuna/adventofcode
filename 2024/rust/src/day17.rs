use aoc::{parsers::int, AOContext};
use itertools::Itertools;
use nom::{bytes::complete::tag, character::complete::one_of, multi::separated_list1, sequence::{preceded, tuple}, Parser};

#[derive(Debug, Clone)]
struct Computer {
    registers: [u64; 3],
    program: Vec<u8>,
}

impl Computer {
    fn combo(&self, operand: u8) -> u64 {
        match operand {
            x@0..=3 => x as _,
            i@4..=6 => self.registers[i as usize - 4],
            _ => unreachable!(),
        }
    }
}

fn parse(input: &str) -> Computer {
    let register = |input| tuple((tag("Register "), one_of("ABC"), tag(": "), int::<u64>, tag("\n"))).map(|(_, _, _, i, _)| i).parse(input);
    let program = preceded(tag("Program: "), separated_list1(tag(","), int));

    let (a, b, c, _, p) = tuple((
        register,
        register,
        register,
        tag("\n"),
        program
    ))(input).unwrap().1;

    Computer { registers: [a, b, c], program: p }
}

fn part1(mut computer: Computer) -> Vec<u8> {
    let mut pc = 0;
    let mut res = Vec::new();

    while let Some(&op) = computer.program.get(pc) {
        let Some(&operand) = computer.program.get(pc + 1) else { break; };
        match op {
            0 => computer.registers[0] = computer.registers[0] >> computer.combo(operand),
            1 => computer.registers[1] = computer.registers[1] ^ operand as u64,
            2 => computer.registers[1] = computer.combo(operand) & 7,
            3 => if computer.registers[0] != 0 {
                pc = operand as _;
                continue;
            }
            4 => computer.registers[1] = computer.registers[1] ^ computer.registers[2],
            5 => res.push((computer.combo(operand) & 7) as u8),
            6 => computer.registers[1] = computer.registers[0] >> computer.combo(operand),
            7 => computer.registers[2] = computer.registers[0] >> computer.combo(operand),
            _ => unreachable!(),
        }
        pc += 2;
    }

    res
}

fn fucking_dfs_again(results: &[u8], a: u64) -> Option<u64> {
    if results.len() == 0 {
        Some(a)
    } else {
        let a = a << 3;
        let possible = (0..8).filter(|b| {
            let a2 = a | b;
            let mut b = *b;
            b = b ^ 1;
            let c = a2 >> b;
            b = b ^ 5;
            b = b ^ c;

            (b & 7) as u8 == results[0]
        });

        for next in possible {
            let res = fucking_dfs_again(&results[1..], a | next);

            if res.is_some() {
                return res;
            }
        }

        None
    }
}

fn part2(computer: &Computer) -> u64 {
    let target = computer.program.iter().cloned().rev().collect_vec();
    fucking_dfs_again(&target, 0).unwrap()
}

pub fn day17(input: String, ctx: &mut AOContext) {
    let computer = parse(&input); 
    ctx.parsing_done();

    let r = part1(computer.clone());
    ctx.submit_part1(format!("{r:?}"));

    ctx.submit_part2(part2(&computer));
}
