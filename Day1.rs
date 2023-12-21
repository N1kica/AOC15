use std::fs;

fn evaluate(c: &char) -> i32 {
    match c {
        '(' => 1,
        _ => -1,
    }
}

fn main() {
    let file = fs::read_to_string("input.txt").expect("Something went wrong reading the file");

    let total_steps: i32 = file.chars().fold(0, |acc, c| {
        acc + evaluate(&c)
    });

    println!("Santa takes {} total steps", total_steps);

    let mut pos: i32 = 0;

    let index = file.chars().enumerate().position(|(_i, c)| {
        pos += evaluate(&c);
        pos <= -1
    });

    match index {
        Some(idx) => println!("Step when santa reaches basement: {}", &idx + 1),
        None => println!("Santa doesn't reach the basement with given instructions"),
    }
}
