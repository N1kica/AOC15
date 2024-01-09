fn part_one() -> i32 {
    aoc::chars("./data/day16.txt", |c| Some(c))
        .iter()
        .fold(0, |acc, x| acc + step(x))
}

fn part_two() -> usize {
    aoc::chars("./data/day16.txt", |c| Some(c))
        .iter()
        .scan(0, |acc, x| Some(std::mem::replace(acc, *acc + step(x))))
        .position(|x| x == -1)
        .unwrap()
}

fn step(inst: &char) -> i32 {
    match inst {
        '(' => 1,
        _ => -1,
    }
}

fn main() {
    println!("Part 1: {}", part_one());
    println!("Part 2: {}", part_two() + 1);
}
