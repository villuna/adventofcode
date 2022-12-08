use raylib::prelude::*;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

const SCALE: i32 = 10;

pub fn day_nine(part: usize, filename: String) {
    let mut file = File::open(filename).unwrap();
    let mut contents = String::new();

    file.read_to_string(&mut contents).unwrap();

    if part == 1 {
        part_one(contents);
    } else {
        part_two_raylib(contents);
    }
}

fn part_one(contents: String) {
    let grid = contents
        .split("\n")
        .filter(|s| !s.is_empty())
        .map(|s| {
            s.chars()
                .filter_map(|c| char::to_digit(c, 10))
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>();

    let mut sum = 0;

    for i in 0..grid.len() as i32 {
        for j in 0..grid[i as usize].len() as i32 {
            let is_low = vec![(-1, 0), (1, 0), (0, -1), (0, 1)]
                .into_iter()
                .filter(|(x, y)| {
                    (i + *x) >= 0
                        && (j + *y) >= 0
                        && (i + *x) < grid.len() as i32
                        && (j + *y) < grid[i as usize].len() as i32
                })
                .map(|(x, y)| ((i + x) as usize, (j + y) as usize))
                .all(|(x, y)| grid[x][y] > grid[i as usize][j as usize]);

            if is_low {
                sum += grid[i as usize][j as usize] + 1;
            }
        }
    }

    println!("The answer is {}", sum);
}

fn get_basin(grid: &Vec<Vec<u32>>, point: (usize, usize)) -> Vec<(usize, usize)> {
    let mut temp_grid = grid
        .clone()
        .into_iter()
        .map(|v| v.iter().map(|x| (*x, false)).collect::<Vec<(u32, bool)>>())
        .collect::<Vec<Vec<(u32, bool)>>>();

    get_basin_helper(&mut temp_grid, point)
}

fn get_basin_helper(
    grid: &mut Vec<Vec<(u32, bool)>>,
    point: (usize, usize),
) -> Vec<(usize, usize)> {
    grid[point.0][point.1].1 = true;

    let mut answer = vec![(point.0, point.1)];

    let i = point.0 as i32;
    let j = point.1 as i32;

    let indices = vec![(-1, 0), (1, 0), (0, -1), (0, 1)]
        .into_iter()
        .filter(|&(x, y)| {
            x + i >= 0 && y + j >= 0 && x + i < grid.len() as i32 && y + j < grid[0].len() as i32
        })
        .map(|(x, y)| ((x + i) as usize, (y + j) as usize))
        .collect::<Vec<(usize, usize)>>();

    for (x, y) in indices {
        if !grid[x][y].1 && grid[x][y].0 != 9 {
            answer.append(&mut get_basin_helper(grid, (x, y)));
        }
    }

    answer
}

// TODO Refactor this bigtime
fn part_two_raylib(contents: String) {
    let grid = contents
        .split("\n")
        .filter(|s| !s.is_empty())
        .map(|s| {
            s.chars()
                .filter_map(|c| char::to_digit(c, 10))
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>();

    let width = grid.len() as i32;
    let height = grid[0].len() as i32;

    let (mut rl, thread) = raylib::init()
        .size(width * SCALE, height * SCALE)
        .title("woooo yeah baby advent of code oooowwwuuuoooohh  im groovin")
        .build();

    let mut basins: HashMap<(usize, usize), Vec<(usize, usize)>> = HashMap::new();

    for i in 0..grid.len() as i32 {
        for j in 0..grid[i as usize].len() as i32 {
            let is_low = vec![(-1, 0), (1, 0), (0, -1), (0, 1)]
                .into_iter()
                .filter(|(x, y)| {
                    (i + *x) >= 0
                        && (j + *y) >= 0
                        && (i + *x) < grid.len() as i32
                        && (j + *y) < grid[i as usize].len() as i32
                })
                .map(|(x, y)| ((i + x) as usize, (j + y) as usize))
                .all(|(x, y)| grid[x][y] > grid[i as usize][j as usize]);

            if is_low {
                let point = (i as usize, j as usize);
                basins.insert(point, vec![]);
            }
        }
    }

    /*
    let mut sizes = basins.values()
        .map(|v| v.len())
        .collect::<Vec<usize>>();

    sizes.sort();

    let answer = sizes.iter().rev().take(3).product::<usize>();

    println!("answer is {}", answer);
    */

    let pallette = (1..11)
        .map(|i| (i * 255 / 10) as u8)
        .map(|i| Color::color_from_hsv(0.0, 0.0, i as f32 / 255.0))
        .collect::<Vec<Color>>();

    //pallette[9] = Color::RED;

    // RGB enabled
    let blue_pallette = (1..11)
        .map(|i| (i * 255 / 10) as u8)
        .map(|i| Color::color_from_hsv(240.0, 1.0, i as f32 / 255.0))
        .collect::<Vec<Color>>();

    let mut timer = 0;
    let mut tick = 5;

    let low_points = basins.keys().map(|&p| p).collect::<Vec<(usize, usize)>>();
    let mut current_point = 0;

    let mut stack = vec![low_points[0]];

    while !rl.window_should_close() {
        if rl.is_key_pressed(KeyboardKey::KEY_F2) {
            rl.take_screenshot(&thread, "day9.png");
        }

        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::WHITE);

        for i in 0..width {
            for j in 0..height {
                d.draw_rectangle(
                    j * SCALE,
                    i * SCALE,
                    SCALE,
                    SCALE,
                    pallette[grid[i as usize][j as usize] as usize],
                );
            }
        }

        for &(i, j) in basins.values().flatten() {
            d.draw_rectangle(
                j as i32 * SCALE,
                i as i32 * SCALE,
                SCALE,
                SCALE,
                blue_pallette[grid[i][j] as usize],
            );
        }

        for &(i, j) in low_points.iter() {
            d.draw_rectangle(
                j as i32 * SCALE,
                i as i32 * SCALE,
                SCALE,
                SCALE,
                blue_pallette[grid[i][j] as usize],
            );
        }

        if let Some(&(i, j)) = stack.last() {
            d.draw_rectangle(
                j as i32 * SCALE,
                i as i32 * SCALE,
                SCALE,
                SCALE,
                Color::GREEN,
            );
        }

        if tick == 0 {
        } else {
            timer += 1;
            if timer == tick {
                if let Some((i, j)) = stack.pop() {
                    basins
                        .get_mut(&low_points[current_point])
                        .unwrap()
                        .push((i, j));

                    let i = i as i32;
                    let j = j as i32;

                    let indices = vec![(-1, 0), (1, 0), (0, -1), (0, 1)]
                        .into_iter()
                        .filter(|&(x, y)| {
                            x + i >= 0
                                && y + j >= 0
                                && x + i < grid.len() as i32
                                && y + j < grid[0].len() as i32
                        })
                        .map(|(x, y)| ((x + i) as usize, (y + j) as usize))
                        .collect::<Vec<(usize, usize)>>();

                    for (x, y) in indices {
                        if grid[x][y] != 9
                            && !basins
                                .get(&low_points[current_point])
                                .unwrap()
                                .contains(&(x, y))
                            && !stack.contains(&(x, y))
                        {
                            stack.push((x, y));
                        }
                    }
                } else {
                    current_point += 1;

                    if current_point == low_points.len() {
                        tick = 0;
                    } else {
                        stack.push(low_points[current_point]);
                    }
                }

                timer = 0;
            }
        }
    }
}

#[allow(unused)]
fn part_two(contents: String) {
    let grid = contents
        .split("\n")
        .filter(|s| !s.is_empty())
        .map(|s| {
            s.chars()
                .filter_map(|c| char::to_digit(c, 10))
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>();

    let mut basins: HashMap<(usize, usize), Vec<(usize, usize)>> = HashMap::new();

    for i in 0..grid.len() as i32 {
        for j in 0..grid[i as usize].len() as i32 {
            let is_low = vec![(-1, 0), (1, 0), (0, -1), (0, 1)]
                .into_iter()
                .filter(|(x, y)| {
                    (i + *x) >= 0
                        && (j + *y) >= 0
                        && (i + *x) < grid.len() as i32
                        && (j + *y) < grid[i as usize].len() as i32
                })
                .map(|(x, y)| ((i + x) as usize, (j + y) as usize))
                .all(|(x, y)| grid[x][y] > grid[i as usize][j as usize]);

            if is_low {
                let point = (i as usize, j as usize);
                basins.insert(point, get_basin(&grid, point));
            }
        }
    }

    let mut sizes = basins.values().map(|v| v.len()).collect::<Vec<usize>>();

    sizes.sort();

    let answer = sizes.iter().rev().take(3).product::<usize>();

    println!("answer is {}", answer);
}
