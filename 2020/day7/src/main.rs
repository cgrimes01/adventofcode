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
    let mut bags: Vec<Bag> = Vec::new();
    for line in input.lines() {
        bags = process_bag_line(&line, bags);
    }
    dbg!(bags);
    5
}

fn append_word(current : &mut String, append : &str) {
    if current != "" {
        current.push_str(" ");
    }
    current.push_str(append);
}

// fn get_bags_containing(colour : String, bags : HashMap<String, Bag>) -> HashMap<String, Bag> {
//     let mut containingbags = HashMap::new();

//     for bag in bags {
//         if bag
//     }
//     containingbags
// }

fn process_bag_line(input : &str, mut bags: Vec<Bag>) -> Vec<Bag> {
    let mut state = "colour";
    let mut contained_colour = String::from("");
    let mut container_colour = String::from("");
    let mut contained_count = 0;
    let mut contents = HashMap::new();
    for word in input.split(" ") {
        match state {
            "colour" => {
                match word {
                    "bags" => {},
                    "contain" => state = "contains",
                    _ => {
                        append_word(&mut container_colour, word);
                    }
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
                                contents.insert(contained_colour, contained_count);
                            }
                            contained_colour = "".to_string();
                            contained_count = 0;
                        } else {
                            append_word(&mut contained_colour, word);
                        }
                    },
                };
            }, 
            _ => {},
        }
    }
    bags.push(Bag { colour: container_colour.clone(), contents: contents});
    bags
}