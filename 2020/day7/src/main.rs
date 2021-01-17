use std::fs;
use std::collections::HashMap;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    println!("Sum of distinct counts = {}", part1(&input, "shiny gold".to_string()));
    // println!("Sum of all counts = {}", part2(&input));
}

#[derive(Debug)]
struct Bag {
    colour: String,
    contents: HashMap<String, usize>,
}

fn part1(input : &String, colour: String) -> usize {
    let mut bags: Vec<Bag> = Vec::new();
    for line in input.lines() {
        bags = process_bag_line(&line, bags);
    }
    let containingbags = get_bags_containing(colour, &bags);
    containingbags.len()
}

fn append_word(current : &mut String, append : &str) {
    if current != "" {
        current.push_str(" ");
    }
    current.push_str(append);
}

fn get_bags_containing(colour : String, bags : &Vec<Bag>) -> HashMap<String, String> {
    let mut containingbags : HashMap<String, String> = HashMap::new();

    for bag in bags {
        match bag.contents.get(&colour) {
            Some(_) => {
                containingbags.insert(bag.colour.clone(), bag.colour.clone());
                let result = get_bags_containing(bag.colour.clone(), &bags);
                containingbags.extend(result);
            },
            _ => {}, 
        }
    }
    containingbags
}

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

#[cfg(test)]
mod part1_tests {
    use super::*;
    #[test]
    fn group_1() {
        let input = "light red bags contain 1 bright white bag, 2 muted yellow bags.\ndark orange bags contain 3 bright white bags, 4 muted yellow bags.\nbright white bags contain 1 shiny gold bag.\nmuted yellow bags contain 2 shiny gold bags, 9 faded blue bags.\nshiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.\ndark olive bags contain 3 faded blue bags, 4 dotted black bags.\nvibrant plum bags contain 5 faded blue bags, 6 dotted black bags.\nfaded blue bags contain no other bags.\ndotted black bags contain no other bags.".to_string();  
        assert_eq!(part1( &input, "shiny gold".to_string()), 4);
    }
}
