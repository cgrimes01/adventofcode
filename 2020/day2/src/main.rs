use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

struct PasswordLine {
    password: String,
    letter: char,
    minimum: usize,
    maximum: usize
}

fn main() {
    let mut passwords: Vec<PasswordLine> = Vec::new();

    if let Ok(lines) = read_lines("input.txt") {
        for line in lines {
            if let Ok(text) = line {
                let space_split = text.split(" ");
                let mut password : &str = "";
                let mut letter : char = ' ';
                let mut minimum : usize = 0;
                let mut maximum : usize = 0;
                for (i, block) in space_split.enumerate() {
                    if i == 0 {
                        let mut num_split = block.split("-");
                        minimum = num_split.nth(0).unwrap().parse().expect("Not a number!");
                        maximum = num_split.nth(0).unwrap().parse().expect("Not a number!");
                    }
                    if i == 1 {
                        letter = block.chars().next().unwrap();
                    }
                    if i == 2 {
                        password = block;
                    }
                }
                // println!("Letter = {}, Password = {}, Minimum = {}, Maximum = {}", letter, password, minimum, maximum);
                passwords.push(PasswordLine {
                    password: String::from(password),
                    letter: letter,
                    minimum: minimum,
                    maximum: maximum
                });
            }
        }
    }
    println!("Valid Passwords = {}", password_checks(passwords));
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn password_checks(passwords : Vec<PasswordLine>) -> i32 {
    let mut valid_passwords : i32 = 0;
    for (_, password_line) in passwords.iter().enumerate() {
        let letter_occurrences = password_line.password.chars().filter(|letter| *letter == password_line.letter).count();
        if letter_occurrences >= password_line.minimum && letter_occurrences <= password_line.maximum {
            valid_passwords = valid_passwords + 1;
        }
    }
    valid_passwords
}
