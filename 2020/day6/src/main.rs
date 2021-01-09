use std::fs;
use std::collections::HashMap;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    println!("Sum of distinct counts = {}", part1(&input));
    println!("Sum of all counts = {}", part2(&input));
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

fn part2(input : &String) -> usize{
    let mut group = HashMap::new();
    let mut total = 0;
    let mut first_group = true;
    for line in input.lines() {
        match line {
            "" => {
                total = total + group.len();
                group = HashMap::new();
                first_group = true;
            }
            _ => {
                match first_group {
                    true => {
                        for c in line.chars() {
                            group.insert(c, c);    
                        }
                        first_group = false;
                        
                    }
                    false => {
                        let mut new_group = HashMap::new();
                        for c in line.chars() {
                            if group.contains_key(&c) {
                                new_group.insert(c, c);
                            }         
                        }
                        group = new_group;
                    }
                }
            }
        }
    }
    total = total + group.len();
    total
}

#[cfg(test)]
mod day2_tests {
    use super::*;
    #[test]
    fn group_1() {
        let input = "abc".to_string();  
        assert_eq!(part2(&input), 3);
    }
    #[test]
    fn group_2() {
        let input = "a\nb\nc".to_string();  
        assert_eq!(part2(&input), 0);
    }
    #[test]
    fn group_3() {
        let input = "ab\nac".to_string();  
        assert_eq!(part2(&input), 1);
    }
}
