use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let mut numbers : Vec<u64> = Vec::new();
    for line in input.lines() {
        numbers.push(line.parse().unwrap());
    };

    println!("First invalid number = {}", find_invalid_number(&numbers, 25));
}

fn find_invalid_number(numbers: &Vec<u64>, preamble_length: usize) -> u64 {
    let mut invalid_found = false; 
    let mut current_index : usize = preamble_length;
    while !invalid_found {
        let mut match_found = false;
        for x in (current_index - preamble_length)..current_index  {
            for y in (current_index - preamble_length)..current_index {
                if x != y && numbers[x] + numbers[y] == numbers[current_index] {
                    match_found = true;
                    break;
                }
            }
            if match_found {
                break; 
            }
        }
        if !match_found {
            invalid_found = true;
        } else {
            current_index = current_index + 1;
        }
    }
    numbers[current_index]
}

#[cfg(test)]
mod part1_tests {
    use super::*;
    #[test]
    fn part_1() {
        let input = fs::read_to_string("testinput.txt").unwrap();
        let mut numbers : Vec<u64> = Vec::new();
        for line in input.lines() {
            numbers.push(line.parse().unwrap());
        };

        let output = find_invalid_number(&numbers, 5);
        assert_eq!(output, 127);
    }
}
