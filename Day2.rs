use std::fs;

fn get_slack(l: i32, w: i32, h: i32) -> i32 {
    if l <= w && l <= h {
        l
    } else if w <= l && w <= h {
        w
    } else {
        h
    }
}

fn get_dimension(line: &str) -> i32 {
    let cords: Vec<i32> = line
        .split('x')
        .filter_map(|x| x.parse().ok())
        .take(3)
        .collect();

    let l = cords[0] * cords[1];
    let w = cords[1] * cords[2];
    let h = cords[2] * cords[0];
    let slack = get_slack(l, w, h);

    return 2 * l + 2 * w + 2 * h + slack;
}

fn main() {
    let file = fs::read_to_string("input.txt").expect("Something went wrong reading the file");

    let contents: i32  = file
        .lines()
        .fold(0, |acc, x| acc + get_dimension(x));

    println!("{}", contents);
}
