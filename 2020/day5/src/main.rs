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
        dbg!(calc);
        dbg!(bound);
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
