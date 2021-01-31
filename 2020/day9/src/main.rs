use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let mut numbers : Vec<u64> = Vec::new();
    for line in input.lines() {
        numbers.push(line.parse().unwrap());
    };

    let invalid_number = get_invalid_number(&numbers, 25);
    println!("First invalid number = {}", invalid_number);
    println!("Encryption weakness = {}", get_encryption_weakness(&numbers, invalid_number));
}

fn get_invalid_number(numbers: &Vec<u64>, preamble_length: usize) -> u64 {
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

fn get_encryption_weakness(numbers: &Vec<u64>, target_total: u64) -> u64 {
    let range = find_range_equal_to_total(numbers, target_total);
    sum_min_and_max(&range)
}

fn find_range_equal_to_total(numbers: &Vec<u64>, target_total: u64) -> Vec<u64> {
    let mut range_found = false; 
    let mut current_index : usize = 0;
    let mut ending_index : usize = 0;

    while !range_found {
        let mut current_total : u64 = 0;
        for x in current_index..numbers.len() - 1  {
            current_total = current_total + numbers[x];
            
            if current_total == target_total {
                range_found = true;
                ending_index = x;
                break;
            }

            if current_total > target_total {
                break;
            }
        }

        if !range_found {
            current_index = current_index + 1;
        }
    }
    numbers[current_index..=ending_index].to_vec()
}

fn sum_min_and_max(numbers : &Vec<u64>) -> u64 {
    let min = numbers.iter().min().unwrap();
    let max = numbers.iter().max().unwrap();

    max + min
}

#[cfg(test)]
mod day9_tests {
    use super::*;
    #[test]
    fn part_1() {
        let input = fs::read_to_string("testinput.txt").unwrap();
        let mut numbers : Vec<u64> = Vec::new();
        for line in input.lines() {
            numbers.push(line.parse().unwrap());
        };

        let output = get_invalid_number(&numbers, 5);
        assert_eq!(output, 127);
    }

    #[test]
    fn part_2() {
        let input = fs::read_to_string("testinput.txt").unwrap();
        let mut numbers : Vec<u64> = Vec::new();
        for line in input.lines() {
            numbers.push(line.parse().unwrap());
        };

        let output = get_encryption_weakness(&numbers, 127);
        assert_eq!(output, 62);
    }
}
