use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut numbers: Vec<i32> = Vec::new();

    if let Ok(lines) = read_lines("input.txt") {
        for line in lines {
            if let Ok(text) = line {
                let val : i32 = text.parse().expect("Not a number!");
                numbers.push(val);
            }
        }
    }
    println!("2020 sum with 2 numbers = {}", get_2020_product_2numbers(&numbers));
    println!("2020 sum with 3 numbers = {}", get_2020_product_3numbers(&numbers));
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn get_2020_product_2numbers(numbers: &Vec<i32>) -> i32 {
    for (i, outerval) in numbers.iter().enumerate() {
        for (y, innerval) in numbers.iter().enumerate() {
            if i != y && outerval + innerval == 2020 {
                return innerval * outerval
            }
        }
    }
    0
}

fn get_2020_product_3numbers(numbers: &Vec<i32>) -> i32 {
    for (i, val1) in numbers.iter().enumerate() {
        for (y, val2) in numbers.iter().enumerate() {
            for (z, val3) in numbers.iter().enumerate() {
                if i != y && i!= z && y!= z && val1 + val2 + val3 == 2020 {
                    return val1 * val2 * val3
                }
            }
        }
    }
    0
}
