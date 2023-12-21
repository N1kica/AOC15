use std::fs;

fn get_rabbon(l: i32, w: i32, h: i32) -> i32 {
    let mut nums = vec![l, w, h];
    nums.sort();
    return 2 * nums[0] + 2 * nums[1] + nums[0] * nums[1] * nums[2]; 
}

fn get_cords(l: i32, w: i32, h: i32, slack: i32) -> i32 {
    return 2 * l * w + 2 * w * h + 2 * h * l + slack;
}

fn get_slack(l: i32, w: i32, h: i32) -> i32 {
    let lw = l * w;
    let wh = w * h;
    let hl = h * l;

    if lw <= wh && lw <= hl {
        lw
    } else if wh <= lw && wh <= hl {
        wh
    } else {
        hl
    }
}

fn get_dimension(line: &str) -> (i32, i32) {
    let cords: Vec<i32> = line
        .split('x')
        .filter_map(|x| x.parse().ok())
        .take(3)
        .collect();

    let slack = get_slack(cords[0], cords[1], cords[2]);
    let rabbon = get_rabbon(cords[0], cords[1], cords[2]);
    let cords = get_cords(cords[0], cords[1], cords[2], slack);

    return (cords, rabbon);
}

fn main() {
    let file = fs::read_to_string("input.txt").expect("Something went wrong reading the file");

    let contents: (i32, i32)  = file
        .lines()
        .fold((0, 0), |acc, x| {
            let dimensions = get_dimension(x);
            (acc.0 + dimensions.0, acc.1 + dimensions.1)
        });

    println!("{:?}", contents);
}
