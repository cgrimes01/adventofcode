use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let max = get_max(&input);
    part1(&input, max);
    // part2(&input, max);
}

// struct Slope {
//     right : u32,
//     down: u32,
// }

// fn part2(input : &String, max : i32) {
//     let slopes = vec![
//         Slope { right: 1, down: 1},
//         Slope { right: 3, down: 1},
//         Slope { right: 5, down: 1},
//         Slope { right: 7, down: 1},
//         Slope { right: 1, down: 2},
//     ];
//     let mut trees : i32 = 0;
//     let mut x : i32 = 0;
    
//     for line in input.lines() {
//         if line.chars().nth(x as usize).unwrap() == '#' {
//             trees = trees + 1;
//         }
//         x = toboggan(max, x, 3);
//     }
//     dbg!(trees);
// }

fn part1(input : &String, max : i32) {
    let mut trees : i32 = 0;
    let mut x : i32 = 0;
    for line in input.lines() {
        if line.chars().nth(x as usize).unwrap() == '#' {
            trees = trees + 1;
        }
        x = toboggan(max, x, 3);
    }
    dbg!(trees);
}

fn get_max(input : &String) -> i32 {
    (input.find('\n').unwrap() as i32) - 2
}

fn toboggan(max : i32, original_x : i32, x_change : i32) -> i32 {
    let mut x = original_x + x_change;
    if x > max {
        x = x - max - 1;
    }
    x
}
