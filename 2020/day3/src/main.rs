use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    part1(&input);
}

fn part1(input : &String) {
    let mut trees : i32 = 0;
    let mut x : i32 = 0;
    for line in input.lines() {
        dbg!(x);
        if line.chars().nth(x as usize).unwrap() == '#' {
            trees = trees + 1;
        }
        x = toboggan(&line, x, 3);
        dbg!(line);
    }
    dbg!(trees);
}

fn toboggan(line : &str, original_x : i32, x_change : i32) -> i32 {
    let max  = (line.len() - 1) as i32;
    let mut x = original_x + x_change;
    if x > max {
        x = x - max - 1;
    }
    x
}
