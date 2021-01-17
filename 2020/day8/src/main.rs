use std::fs;
use std::collections::HashSet;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let instructions = get_instructions(&input);
    println!("Part 1 = {}", part1(&instructions));
    println!("Part 2 = {}", part2(&instructions));
}

#[derive(Debug, Copy, Clone)]
enum InstructionType {
    Nop,
    Acc,
    Jmp,
}

#[derive(Debug)]
struct Instruction {
    instruction_type: InstructionType,
    value: i32,
}

fn get_instructions(input : &String) -> Vec<Instruction> {
    let mut instructions: Vec<Instruction> = Vec::new();
    for line in input.lines() {
        let mut instruction_type = InstructionType::Nop;
        let mut instruction_value : i32 = 0;
        for word in line.split(" ") {
            match word {
                "nop" => {
                    instruction_type = InstructionType::Nop;
                },
                "acc" => {
                    instruction_type = InstructionType::Acc;
                },
                "jmp" => {
                    instruction_type = InstructionType::Jmp;
                },
                _ => {
                    instruction_value = word.parse().unwrap();
                },
            }
        }
        instructions.push( Instruction { instruction_type: instruction_type, value: instruction_value })
    }
    instructions
}

fn part1(input : &Vec<Instruction>) -> i32 {
    let mut processed_instructions: HashSet<usize> = HashSet::new();
    let mut acc = 0;
    let mut current_instruction = 0;
    let mut terminate = false;

    while terminate == false {
        processed_instructions.insert(current_instruction);
        match input[current_instruction].instruction_type {
            InstructionType::Nop => {
                current_instruction = current_instruction + 1;
            },
            InstructionType::Acc => {
                acc = acc + input[current_instruction].value;
                current_instruction = current_instruction + 1;
            },
            InstructionType::Jmp => {
                current_instruction = ((current_instruction as i32) + input[current_instruction].value) as usize;
            },
        }
        if processed_instructions.contains(&current_instruction) {
            terminate = true;
        }
    }
    acc
}

fn part2(input : &Vec<Instruction>) -> i32 {
    let mut processed_instructions: HashSet<usize>;
    let mut acc = 0;
    let mut current_instruction : usize = 0;
    let mut terminate : bool;
    let correct_end = input.len();
    let mut modified_instruction_count = 0;
    let mut current_modifiable_instruction_count;
    let mut loop_modified;

    while !(&current_instruction == &correct_end) {
        acc = 0;
        current_instruction = 0;
        terminate = false;
        current_modifiable_instruction_count = 0;
        processed_instructions = HashSet::new();
        loop_modified = false;
        while terminate == false {
            processed_instructions.insert(current_instruction);
            match input[current_instruction].instruction_type {
                InstructionType::Nop => {
                    current_modifiable_instruction_count = current_modifiable_instruction_count + 1;
                    if current_modifiable_instruction_count == (modified_instruction_count  + 1) && loop_modified == false {
                        current_instruction = ((current_instruction as i32) + input[current_instruction].value) as usize;
                        modified_instruction_count = modified_instruction_count + 1;
                        loop_modified = true;
                    } else {
                        current_instruction = current_instruction + 1;
                    }
                },
                InstructionType::Acc => {
                    acc = acc + input[current_instruction].value;
                    current_instruction = current_instruction + 1;
                },
                InstructionType::Jmp => {
                    current_modifiable_instruction_count = current_modifiable_instruction_count + 1;
                    if current_modifiable_instruction_count == (modified_instruction_count  + 1) && loop_modified == false {
                        current_instruction = current_instruction + 1;
                        modified_instruction_count = modified_instruction_count + 1;
                        loop_modified = true;
                    } else {
                        current_instruction = ((current_instruction as i32) + input[current_instruction].value) as usize;
                    }  
                },
            }
            if processed_instructions.contains(&current_instruction) || &current_instruction == &correct_end {
                terminate = true;
            }
        }
    }
    acc
}

#[cfg(test)]
mod part2_tests {
    use super::*;
    #[test]
    fn part_1() {
        let input = "nop +0\nacc +1\njmp +4\nacc +3\njmp -3\nacc -99\nacc +1\njmp -4\nacc +6".to_string();  
        let instructions = get_instructions(&input);
        assert_eq!(part2(&instructions), 8);
    }
}

