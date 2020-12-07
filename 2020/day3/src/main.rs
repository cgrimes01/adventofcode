use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    part1(&input);
    part2(&input);
}

struct Slope {
    right : u32,
    down: u32,
}

fn part2(input : &String) {
    let slopes = vec![
        Slope { right: 1, down: 1},
        Slope { right: 3, down: 1},
        Slope { right: 5, down: 1},
        Slope { right: 7, down: 1},
        Slope { right: 1, down: 2},
    ];
    let mut slope_product : u64 = 1;
    
    for slope in slopes {
        slope_product = slope_product * get_trees(input, slope) as u64;
    }
    dbg!(slope_product);
}

fn part1(input : &String) {
    let mut trees : u32 = 0;
    let mut x : u32 = 0;
    let max = get_max(&input);
    for line in input.lines() {
        if line.chars().nth(x as usize).unwrap() == '#' {
            trees = trees + 1;
        }
        x = toboggan_right(max, x, 3);
    }
    dbg!(trees);
}

fn get_trees(input : &String, slope : Slope) -> u32 {
    let mut trees : u32 = 0;
    let mut x : u32 = 0;
    let max = get_max(&input);

    for line in input.lines().step_by(slope.down as usize) {
        if line.chars().nth(x as usize).unwrap() == '#' {
            trees = trees + 1;
        }
        x = toboggan_right(max, x, slope.right);
    }
    trees
}

fn get_max(input : &String) -> u32 {
    (input.find('\n').unwrap() as u32) - 2
}

fn toboggan_right(max : u32, original_x : u32, right : u32) -> u32 {
    let mut x = original_x + right;
    if x > max {
        x = x - max - 1;
    }
    x
}
