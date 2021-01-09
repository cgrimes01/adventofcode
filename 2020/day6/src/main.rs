use std::fs;
use std::collections::HashMap;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    println!("Sum of counts = {}", part1(&input));
    // println!("Valid passports part 2 = {}", part2(&input));
}

fn part1(input : &String) -> usize{
    let mut group = HashMap::new();
    let mut total = 0;
    for line in input.lines() {
        match line {
            "" => {
                total = total + group.len();
                group = HashMap::new();
            }
            _ => {
                for c in line.chars() {
                    if !group.contains_key(&c) {
                        group.insert(c, c);
                    }         
                }
            }
        }
    }
    total = total + group.len();
    total
}