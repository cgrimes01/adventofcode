use std::collections::HashMap;
use std::fs;
use regex::Regex;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    println!("Valid passports = {}", part1(&input));
}

struct Passport {
    key_values : HashMap<String, String>,
}

impl Passport {
    fn validate(&self) -> bool {
        self.key_values.contains_key("byr") 
            && self.key_values.contains_key("iyr")
            && self.key_values.contains_key("eyr")
            && self.key_values.contains_key("hgt")
            && self.key_values.contains_key("hcl")
            && self.key_values.contains_key("ecl")
            && self.key_values.contains_key("pid")
    }
}

fn part1(input : &String) -> u32{
    let re = Regex::new(r"([^ \n\r]*:[^ \n\r]*)").unwrap();
    let mut passport = Passport { key_values: HashMap::new() };
    let mut valid_passports = 0;
    for line in input.lines() {
        match line {
            "" => {
                dbg!(passport.validate());
                if let true = passport.validate() {
                    valid_passports = valid_passports + 1;
                };
                passport = Passport { key_values: HashMap::new() };
            }
            _ => {
                for cap in re.captures_iter(line) {
                    let vec = &cap[1].split(':').collect::<Vec<&str>>();
                    passport.key_values.insert(vec[0].to_string(), vec[1].to_string());           
                }
            }
        }
    }
    if let true = passport.validate() {
        valid_passports = valid_passports + 1;
    };
    valid_passports
}
