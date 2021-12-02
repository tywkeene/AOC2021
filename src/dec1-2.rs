use std::fs;

fn collect_window_sums(input: Vec<i32>, win_size: usize) -> Vec<i32> {
    let mut i = 0;
    let mut sum = 0;
    let mut arr: Vec<i32> = Vec::new();

    loop {
        let win: Vec<i32> = input[i..i+win_size].to_vec();

        for value in win {
            sum += value;
        }

        arr.push(sum);
        sum = 0;

        if (i + 3) >= input.len() {
            break;
        }
        i += 1;
    }

    println!("Collected {} windows of size {}", arr.len(), win_size);

    arr
}

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
    let sums = collect_window_sums(arr, 3);
    let depth_increases = find_depth_increases(sums);

    println!("{} depth increases", depth_increases);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn vectors_eq<T: PartialEq>(a: &Vec<T>, b: &Vec<T>) -> bool {
        let matching = a.iter().zip(b.iter()).filter(|&(a, b)| a == b).count();
        matching == a.len() && matching == b.len()
    }

    #[test]
    fn test_collect_window_sums(){
        let depths: Vec<i32> = [199, 200, 208, 210, 200, 207, 240, 269, 260, 263].to_vec();
        let sums: Vec<i32> = [607, 618, 618, 617, 647, 716, 769, 792].to_vec();

        assert_eq!(vectors_eq(&depths, &sums), true);
    }

    #[test]
    fn test_find_depth_increases_window() {
        let depths: Vec<i32> = [199, 200, 208, 210, 200, 207, 240, 269, 260, 263].to_vec();
        let sums = collect_window_sums(depths, 3);

        println!("Sums: {:?}", sums);
        assert_eq!(find_depth_increases(sums), 5);
    }
}
