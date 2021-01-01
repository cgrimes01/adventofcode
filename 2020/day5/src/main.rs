fn main() {
    println!("Hello, world!");
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