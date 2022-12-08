use std::collections::HashSet;
use std::fs::File;
use std::io::Read;

#[derive(Debug)]
struct Board(Vec<Vec<(u8, bool)>>);

#[derive(Debug)]
struct Bingo {
    boards: Vec<Board>,
    numbers: Vec<u8>,
}

impl Board {
    fn from_string(string: String) -> Board {
        let board: Vec<Vec<(u8, bool)>> = string
            .split("\n")
            .map(|s| {
                s.to_string()
                    .split(" ")
                    .filter_map(|s| u8::from_str_radix(s, 10).ok())
                    .map(|num| (num, false))
                    .collect::<Vec<(u8, bool)>>()
            })
            .filter(|v| !v.is_empty())
            .collect();

        Board(board)
    }

    fn update(&mut self, num_called: u8) {
        for (num, status) in self.0.iter_mut().flatten() {
            if *num == num_called {
                *status = true;
            }
        }
    }

    fn has_bingo(&self) -> bool {
        // Check rows
        for row in self.0.iter() {
            if row.iter().all(|entry| entry.1) {
                return true;
            }
        }

        // Check columns
        for i in 0..self.0[0].len() {
            if self.0.iter().all(|row| row[i].1) {
                return true;
            }
        }

        false
    }
}

impl Bingo {
    fn from_string(contents: String) -> Bingo {
        let mut splits = contents.split("\n\n").map(|s| s.to_string()).into_iter();

        let numbers: Vec<u8> = splits
            .next()
            .unwrap()
            .split(",")
            .filter_map(|num| u8::from_str_radix(num, 10).ok())
            .collect();

        let boards: Vec<Board> = splits.map(|s| Board::from_string(s.to_string())).collect();

        Bingo { boards, numbers }
    }
}

pub fn day_four(part: usize, filename: String) {
    let mut file = File::open(filename).unwrap();
    let mut contents = String::new();

    file.read_to_string(&mut contents).unwrap();

    if part == 1 {
        part_one(contents);
    } else {
        part_two(contents);
    }
}

fn part_one(contents: String) {
    let mut bingo = Bingo::from_string(contents);
    let mut winner: Option<&Board> = None;
    let mut last_num = 0;

    'outer: for num in bingo.numbers {
        println!("num: {}", num);
        for (index, board) in bingo.boards.iter_mut().enumerate() {
            board.update(num);
            if board.has_bingo() {
                winner = Some(&bingo.boards[index]);
                last_num = num;
                break 'outer;
            }
        }
    }

    let sum = winner
        .unwrap()
        .0
        .iter()
        .flatten()
        .filter_map(|(num, status)| {
            println!("{:?}", (num, status));
            if *status {
                None
            } else {
                Some(*num as usize)
            }
        })
        .collect::<Vec<usize>>();

    let sum: usize = sum.iter().sum();

    println!("The score is {}", sum * last_num as usize);
}

fn part_two(contents: String) {
    let mut bingo = Bingo::from_string(contents);
    let mut loser: Option<&Board> = None;
    let mut last_num = 0;
    let mut have_won: HashSet<usize> = HashSet::new();

    for num in bingo.numbers {
        let mut last_won = 0;

        for (index, board) in bingo.boards.iter_mut().enumerate() {
            board.update(num);

            if board.has_bingo() {
                if !have_won.contains(&index) {
                    last_won = index;
                    have_won.insert(index);
                }
            }
        }

        if bingo.boards.iter().all(|b| b.has_bingo()) {
            loser = Some(&bingo.boards[last_won]);
            last_num = num;
            break;
        }
    }

    let sum = loser
        .unwrap()
        .0
        .iter()
        .flatten()
        .filter_map(|(num, status)| {
            println!("{:?}", (num, status));
            if *status {
                None
            } else {
                Some(*num as usize)
            }
        })
        .collect::<Vec<usize>>();

    let sum: usize = sum.iter().sum();

    println!("The score is {}", sum * last_num as usize);
}
