use std::fs;
use std::collections::HashMap;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    println!("Sum of distinct counts = {}", part1(&input));
    // println!("Sum of all counts = {}", part2(&input));
}

#[derive(Debug)]
struct Bag {
    colour: String,
    contents: HashMap<String, usize>,
}

fn part1(input : &String) -> usize {
    // let mut bags = Vec::new();
    for line in input.lines() {
        dbg!(process_bag_line(&line));
    }
    5
}

fn process_bag_line(input : &str) -> Bag {
    let mut bag = Bag { colour: "".to_string(), contents: HashMap::new()};
    let mut state = "colour";
    let mut contained_colour = "".to_string();
    let mut contained_count = 0;
    for word in input.split(" ") {
        match state {
            "colour" => {
                if word == "bags" || word == "contains" {
                    state = "contains";
                } else {
                    bag.colour = bag.colour + " " + word;
                }
            },
            "contains" => {
                match word.parse::<usize>() {
                    Ok(number)  => {
                        contained_count = number;
                    },
                    Err(_) => {
                        if word.contains("bag") {
                            if contained_count != 0 {
                                bag.contents.insert(contained_colour.to_string(), contained_count);
                            }
                            contained_colour = "".to_string();
                            contained_count = 0;
                        } else {
                            contained_colour = contained_colour + " " + word;
                        }
                    },
                };
            },
            _ => {},
        }
    }
    bag
}