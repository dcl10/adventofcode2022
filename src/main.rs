use std::fs;

struct Elf {
    id: i32,
    calories: i32,
}

fn main() {
    let mut elves: Vec<Elf> = Vec::new();
    let mut calorie_totals: Vec<i32> = Vec::new();

    let file = fs::read_to_string("data/day1_part1.txt").expect("Couldn't open input file.");

    let mut accumulator = 0;
    for line in file.split("\n") {
        match line.parse::<i32>() {
            Ok(value) => accumulator += value,
            Err(_) => {
                calorie_totals.push(accumulator);
                accumulator = 0;
            }
        };
    }

    for (i, total) in calorie_totals.iter().enumerate() {
        elves.push(Elf {
            id: i as i32,
            calories: *total,
        })
    }

    elves.sort_by(|a, b| {a.calories.cmp(&b.calories)});
    let highest_elf = elves.last().unwrap();

    println!(
        "Elf {} has the most calories with {}",
        highest_elf.id, highest_elf.calories
    )
}
