pub fn day_one(input: String) {
    let mut elves = vec![0];

    for line in input.lines() {
        if line.is_empty() {
            elves.push(0);
        } else {
            *elves.last_mut().unwrap() += line.parse::<usize>().unwrap();
        }
    }

    elves.sort_by(|x, y| y.cmp(x));
    let max = elves[0];
    let top_three: usize = elves.iter().take(3).sum();

    println!(
        "The elf with the most calories is carrying {} calories",
        max
    );
    println!("The top three elves are carrying {} calories", top_three);
}
