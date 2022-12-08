pub fn day_two(input: String) {
    let input = input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            line.split(" ")
                .map(|c| match c {
                    "A" | "X" => 1,
                    "B" | "Y" => 2,
                    "C" | "Z" => 3,
                    _ => panic!("Invalid input"),
                })
                .collect::<Vec<i32>>()
        })
        .map(|vec| (vec[0], vec[1]))
        .collect();

    println!("Part 1: {}", part_one(&input));
    println!("Part 2: {}", part_two(&input));
}

fn part_one(input: &Vec<(i32, i32)>) -> i32 {
    input
        .iter()
        .map(|&(elf_move, my_move)| {
            my_move
                + if elf_move % 3 == my_move - 1 {
                    6
                } else if elf_move == my_move {
                    3
                } else {
                    0
                }
        })
        .sum()
}

fn part_two(input: &Vec<(i32, i32)>) -> i32 {
    input
        .iter()
        .map(|&(elf_move, outcome)| {
            if outcome == 1 {
                (elf_move - 2).rem_euclid(3) + 1
            } else if outcome == 2 {
                elf_move + 3
            } else {
                (elf_move % 3) + 7
            }
        })
        .sum()
}
