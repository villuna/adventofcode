use itertools::Itertools;
use take_until::TakeUntilExt;

// giving me ptsd from my attempt to make a tree yesterday...
#[derive(Debug)]
struct Forest {
    grid: Vec<Vec<i8>>,
    width: usize,
    height: usize,
}

impl Forest {
    fn parse(input: &str) -> Forest {
        let lines = input.lines().filter(|line| !line.is_empty()).collect_vec();

        Forest {
            width: lines[0].len(),
            height: lines.len(),
            grid: lines
                .into_iter()
                .map(|line| {
                    line.chars()
                        .map(|char| char.to_digit(10).unwrap() as i8)
                        .collect_vec()
                })
                .collect_vec(),
        }
    }

    fn number_visible(&self) -> usize {
        // Four grids of zeros with the same dimensions as the forest
        let mut grids = vec![vec![vec![false; self.width]; self.height]; 4];

        // They say don't repeat yourself
        // but adam neely says "repetition legitimises"
        // and adam neely is always right

        let mut max = -1;

        for y in 0..self.height {
            for x in 0..self.width {
                if self.grid[y][x] > max {
                    max = self.grid[y][x];
                    grids[0][y][x] = true;
                }
            }
        }

        for x in 0..self.width {
            max = -1;
            for y in 0..self.height {
                if self.grid[y][x] > max {
                    max = self.grid[y][x];
                    grids[1][y][x] = true;
                }
            }
        }


        for y in 0..self.height {
            max = -1;
            for x in (0..self.width).rev() {
                if self.grid[y][x] > max {
                    max = self.grid[y][x];
                    grids[2][y][x] = true;
                }
            }
        }

        for x in 0..self.width {
            max = -1;
            for y in (0..self.height).rev() {
                if self.grid[y][x] > max {
                    max = self.grid[y][x];
                    grids[3][y][x] = true;
                }
            }
        }

        (0..self.height).cartesian_product(0..self.width)
            .map(|(y, x)| grids.iter().any(|grid| grid[y][x]))
            .map(|b| b as usize)
            .sum()
    }

    fn view_score(&self, pos: (usize, usize)) -> usize {
        // Pos is going to be (y, x) just because of how it be
        
        // I've already used the word "height" so I'll just write it in another language
        // level 99 sigma female code aesthetics
        let takasa = self.grid[pos.0][pos.1];

        let scores = [
            ((pos.1 + 1)..self.width).take_until(|&x| self.grid[pos.0][x as usize] >= takasa).count(),
            ((pos.0 + 1)..self.height).take_until(|&y| self.grid[y as usize][pos.1] >= takasa).count(),
            (0..pos.1).rev().take_until(|&x| self.grid[pos.0][x as usize] >= takasa).count(),
            (0..pos.0).rev().take_until(|&y| self.grid[y as usize][pos.1] >= takasa).count(),
        ];

        scores.iter().product()
    }

    fn max_view_score(&self) -> usize {
        (0..self.height).cartesian_product(0..self.width)
            .map(|p| self.view_score(p))
            .max()
            .unwrap()
    }
}

pub fn day_eight(input: String) {
    let forest = Forest::parse(&input);

    // Part 1
    println!("{}", forest.number_visible());

    // Part 2
    println!("{}", forest.max_view_score());
}
