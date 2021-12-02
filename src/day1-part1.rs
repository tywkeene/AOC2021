use std::fs;

fn find_depth_increases(input: Vec<i32>) -> i32 {
    let mut prev_depth: i32 = 0;
    let mut depth_increases = 0;

    for value in input {
        if prev_depth > 0 && value > prev_depth {
            depth_increases += 1;
        }
        prev_depth = value;
    }

    depth_increases
}

fn convert_strings(filename: String) -> Vec<i32> {
    let input = fs::read(filename).expect("Failed to read file");
    let depths = String::from_utf8(input).unwrap();
    let split: Vec<&str> = depths.split('\n').filter(|s| !s.is_empty()).collect();
    let mut output: Vec<i32> = Vec::new();

    for i in split {
        match i.parse() {
            Ok(value) => {
                output.push(value);
            },
            Err(e) => println!("Failed to parse int: {}", e),
        }
    }
    output
}

fn main() {
    let arr = convert_strings("data/depths.txt".to_string());
    let depth_increases = find_depth_increases(arr);
    println!("{} depth increases", depth_increases);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_depth_increases() {
        let depths: Vec<i32> = [199, 200, 208, 210, 200, 207, 240, 269, 260, 263].to_vec();
        assert_eq!(find_depth_increases(depths), 7);
    }
}
