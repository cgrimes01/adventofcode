use std::collections::HashMap;
use std::fs;
use regex::Regex;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    println!("Valid passports part 1 = {}", part1(&input));
    println!("Valid passports part 2 = {}", part2(&input));
}

struct Passport {
    key_values : HashMap<String, String>,
}

impl Passport {
    fn present_and_valid(&self) -> bool {
        self.fields_present() && self.validate()
    }

    fn fields_present(&self) -> bool {
        self.key_values.contains_key("byr") 
            && self.key_values.contains_key("iyr")
            && self.key_values.contains_key("eyr")
            && self.key_values.contains_key("hgt")
            && self.key_values.contains_key("hcl")
            && self.key_values.contains_key("ecl")
            && self.key_values.contains_key("pid")
    }

    fn validate_byr(&self) -> bool {
        let byr = self.key_values.get("byr").unwrap().parse::<u32>().unwrap();
        byr >= 1920 && byr <= 2002 
    }

    fn validate_iyr(&self) -> bool {
        let iyr = self.key_values.get("iyr").unwrap().parse::<u32>().unwrap();
        iyr >= 2010 && iyr <= 2020
    }

    fn validate_eyr(&self) -> bool {
        let eyr = self.key_values.get("eyr").unwrap().parse::<u32>().unwrap();
        eyr >= 2020 && eyr <= 2030
    }

    fn validate_hgt(&self) -> bool {
        let hgt = self.key_values.get("hgt").unwrap();
        let valid_hgt = 
            match hgt.chars().rev().take(2).collect::<String>().as_str() {
                "mc" => {
                    let cm = hgt.chars().take(hgt.chars().count() - 2).collect::<String>().parse::<u32>().unwrap();
                    cm >= 150 && cm <= 193
                }
                "ni" => {
                    let inches = hgt.chars().take(hgt.chars().count() - 2).collect::<String>().parse::<u32>().unwrap();
                    inches >= 59 && inches <= 76
                }
                _ => {
                    false
                }
            };
        valid_hgt
    }

    fn validate_hcl(&self) -> bool {
        let hcl = self.key_values.get("hcl").unwrap();
        let hcl_re = Regex::new(r"#[0-9a-f]{6}").unwrap();
        hcl_re.is_match(hcl)
    }

    fn validate_ecl(&self) -> bool {
        let ecl = self.key_values.get("ecl").unwrap();
        let valid_ecl = vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
        valid_ecl.contains(&ecl.as_str())
    }

    fn validate_pid(&self) -> bool {
        let pid = self.key_values.get("pid").unwrap();
        let pid_re = Regex::new(r"^[0-9]{9}$").unwrap();
        pid_re.is_match(pid)
    }

    fn validate(&self) -> bool {
        self.validate_byr() &&
            self.validate_iyr() &&
            self.validate_eyr() &&
            self.validate_hgt() &&
            self.validate_hcl() &&
            self.validate_ecl() &&
            self.validate_pid() 
    }
}

fn part1(input : &String) -> u32{
    let re = Regex::new(r"([^ \n\r]*:[^ \n\r]*)").unwrap();
    let mut passport = Passport { key_values: HashMap::new() };
    let mut valid_passports = 0;
    for line in input.lines() {
        match line {
            "" => {
                if let true = passport.fields_present() {
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
    if let true = passport.fields_present() {
        valid_passports = valid_passports + 1;
    };
    valid_passports
}

fn part2(input : &String) -> u32{
    let re = Regex::new(r"([^ \n\r]*:[^ \n\r]*)").unwrap();
    let mut passport = Passport { key_values: HashMap::new() };
    let mut valid_passports = 0;
    for line in input.lines() {
        match line {
            "" => {
                if let true = passport.present_and_valid() {
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
    if let true = passport.present_and_valid() {
        valid_passports = valid_passports + 1;
    };
    valid_passports
}

#[cfg(test)]
mod byr_tests {
    use super::*;
    #[test]
    fn byr_2002() {
        let mut passport = Passport { key_values: HashMap::new() };
        passport.key_values.insert("byr".to_string(), "2002".to_string());  
        assert_eq!(passport.validate_byr(), true);
    }
    #[test]
    fn byr_2003() {
        let mut passport = Passport { key_values: HashMap::new() };
        passport.key_values.insert("byr".to_string(), "2003".to_string());  
        assert_eq!(passport.validate_byr(), false);
    }
}

#[cfg(test)]
mod hgt_tests {
    use super::*;
    #[test]
    fn hgt_60in() {
        let mut passport = Passport { key_values: HashMap::new() };
        passport.key_values.insert("hgt".to_string(), "60in".to_string());  
        assert_eq!(passport.validate_hgt(), true);
    }
    #[test]
    fn hgt_190cm() {
        let mut passport = Passport { key_values: HashMap::new() };
        passport.key_values.insert("hgt".to_string(), "190cm".to_string());  
        assert_eq!(passport.validate_hgt(), true);
    }
    #[test]
    fn hgt_190in() {
        let mut passport = Passport { key_values: HashMap::new() };
        passport.key_values.insert("hgt".to_string(), "190in".to_string());  
        assert_eq!(passport.validate_hgt(), false);
    }
    #[test]
    fn hgt_190() {
        let mut passport = Passport { key_values: HashMap::new() };
        passport.key_values.insert("hgt".to_string(), "190in".to_string());  
        assert_eq!(passport.validate_hgt(), false);
    }
}

#[cfg(test)]
mod hcl_tests {
    use super::*;
    #[test]
    fn hcl_1() {
        let mut passport = Passport { key_values: HashMap::new() };
        passport.key_values.insert("hcl".to_string(), "#123abc".to_string());  
        assert_eq!(passport.validate_hcl(), true);
    }
    #[test]
    fn hcl_2() {
        let mut passport = Passport { key_values: HashMap::new() };
        passport.key_values.insert("hcl".to_string(), "#123abz".to_string());  
        assert_eq!(passport.validate_hcl(), false);
    }
    #[test]
    fn hcl_3() {
        let mut passport = Passport { key_values: HashMap::new() };
        passport.key_values.insert("hcl".to_string(), "123abc".to_string());  
        assert_eq!(passport.validate_hcl(), false);
    }
}

#[cfg(test)]
mod ecl_tests {
    use super::*;
    #[test]
    fn ecl_brn() {
        let mut passport = Passport { key_values: HashMap::new() };
        passport.key_values.insert("ecl".to_string(), "brn".to_string());  
        assert_eq!(passport.validate_ecl(), true);
    }
    #[test]
    fn ecl_wat() {
        let mut passport = Passport { key_values: HashMap::new() };
        passport.key_values.insert("ecl".to_string(), "wat".to_string());  
        assert_eq!(passport.validate_ecl(), false);
    }
}

#[cfg(test)]
mod pid_tests {
    use super::*;
    #[test]
    fn pid_1() {
        let mut passport = Passport { key_values: HashMap::new() };
        passport.key_values.insert("pid".to_string(), "000000001".to_string());  
        assert_eq!(passport.validate_pid(), true);
    }
    #[test]
    fn pid_2() {
        let mut passport = Passport { key_values: HashMap::new() };
        passport.key_values.insert("pid".to_string(), "0123456789".to_string());  
        assert_eq!(passport.validate_pid(), false);
    }
}
