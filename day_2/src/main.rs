use std::fs;

fn sum_amounts<'a, I: Iterator<Item = &'a str>>(directions: I) -> usize {
    directions
        .map(|line| line.split(' '))
        .map(|split_line| split_line.collect::<Vec<_>>()[1])
        .map(|last_char| last_char.parse::<usize>())
        .map(|x| x.expect("Something went wrong parsing a number"))
        .sum()
}

fn main() {
    let input = fs::read_to_string("input.txt")
        .expect("Something went wrong reading the file");

    let forward = input.lines().filter(|line| line.starts_with("forward"));
    let up = input.lines().filter(|line| line.starts_with("up"));
    let down = input.lines().filter(|line| line.starts_with("down"));

    let total_forward = sum_amounts(forward);
    let total_depth = sum_amounts(down) - sum_amounts(up);

    println!(
        "Submarine depth is {}, horizontal position is {}, multiplied: {}",
        total_depth,
        total_forward,
        total_depth * total_forward
    )
}
