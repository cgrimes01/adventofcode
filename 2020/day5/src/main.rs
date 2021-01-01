use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    println!("Highest seat id = {}", part1(&input));
}

fn part1(input : &String) -> u32 {
    let mut max_seat_id = 0;
    for line in input.lines() {
        let position = get_position(line, 128, 8);
        if position.seat_id > max_seat_id {
            max_seat_id = position.seat_id;
        }
    }
    max_seat_id
}

#[derive(Debug, PartialEq, Copy, Clone)]
struct Position {
    row : u32,
    column : u32,
    seat_id : u32,
}

#[derive(Debug, Copy, Clone)]
struct Bounds {
    upper : u32,
    lower : u32,
}

impl Bounds {
    fn select_upper_half(&mut self) {
        let mid = (self.upper - self.lower) / 2;
        self.lower = self.lower + mid + 1
    }
    fn select_lower_half(&mut self) {
        let mid = (self.upper - self.lower) / 2;
        self.upper = self.lower + mid
    }
}

fn get_row(row_code : &str, rows : u32) -> u32 {
    let mut bound = Bounds { upper: rows - 1, lower: 0 };
    for calc in row_code.chars() {
        match calc {
            'F' => bound.select_lower_half(),
            'B' => bound.select_upper_half(),
            _ => {}
        } 
    }
    bound.upper
}

fn get_col(col_code : &str, cols : u32) -> u32 {
    let mut bound = Bounds { upper: cols - 1, lower: 0 };
    for calc in col_code.chars() {
        match calc {
            'L' => bound.select_lower_half(),
            'R' => bound.select_upper_half(),
            _ => {}
        } 
    }
    bound.upper
}

fn get_position(input: &str, rows: u32, cols: u32) -> Position {
    let row_code = &input[0..7];
    let col_code = &input[7..10];
    let row = get_row(row_code, rows);
    let col = get_col(col_code, cols);
    let seat_id = (row * 8) + col;
    let position = Position { row: row, column: col, seat_id: seat_id};
    position    
}

#[cfg(test)]
mod row_tests {
    use super::*;
    #[test]
    fn row_1() {
        let input = "FBFBBFF";  
        assert_eq!(get_row(input, 128), 44);
    }
    #[test]
    fn row_2() {
        let input = "BFFFBBF";  
        assert_eq!(get_row(input, 128), 70);
    }
    #[test]
    fn row_3() {
        let input = "FFFBBBF";  
        assert_eq!(get_row(input, 128), 14);
    }
    #[test]
    fn row_4() {
        let input = "BBFFBBF";  
        assert_eq!(get_row(input, 128), 102);
    }
}

#[cfg(test)]
mod col_tests {
    use super::*;
    #[test]
    fn col_1() {
        let input = "RLR";  
        assert_eq!(get_col(input, 8), 5);
    }
    #[test]
    fn col_2() {
        let input = "RRR";  
        assert_eq!(get_col(input, 8), 7);
    }
    #[test]
    fn col_3() {
        let input = "RLL";  
        assert_eq!(get_col(input, 8), 4);
    }
}

#[cfg(test)]
mod position_tests {
    use super::*;
    #[test]
    fn position_1() {
        let input = "FBFBBFFRLR";
        let expected = Position { row: 44, column: 5, seat_id: 357 };
        let result = get_position(input, 128, 8);
        assert_eq!(result, expected);
    }
    #[test]
    fn position_2() {
        let input = "BFFFBBFRRR";
        let expected = Position { row: 70, column: 7, seat_id: 567 };
        let result = get_position(input, 128, 8);
        assert_eq!(result, expected);
    }
    #[test]
    fn position_3() {
        let input = "FFFBBBFRRR";
        let expected = Position { row: 14, column: 7, seat_id: 119 };
        let result = get_position(input, 128, 8);
        assert_eq!(result, expected);
    }
    #[test]
    fn position_4() {
        let input = "BBFFBBFRLL";
        let expected = Position { row: 102, column: 4, seat_id: 820 };
        let result = get_position(input, 128, 8);
        assert_eq!(result, expected);
    }
}
