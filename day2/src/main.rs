use std::env;
use std::fs;

fn read_file_to_string(filepath: String) -> String {
    let contents = fs::read_to_string(filepath)
        .expect("Should have been able to read the file");

    return contents
}

fn is_repeated_twice_digits(num_str: String) -> bool {
    let size  = num_str.len();
    if size % 2 != 0 || size == 0{
        return false;
    }

    if !num_str.chars().all(|c| c.is_numeric()) {
        return false;
    }

    let mid = size/2;

    return num_str[..mid] == num_str[mid..];
}

fn solve(data: String) {
    let mut answer: i64 = 0;

    let ranges: Vec<&str> = data.split(",").collect();
    for r in ranges {
        let mut iter = r.split("-");
        let min_str:&str = iter.next().unwrap();
        let max_str:&str = iter.next().unwrap();

        let min: i64 = min_str.parse().unwrap();
        let max: i64 = max_str.parse().unwrap();

        for number in min..=max {
            // println!("Number {} {}", number, is_repeated_twice_digits(number.to_string()));
            if is_repeated_twice_digits(number.to_string()) {
                answer += number;
            }
        }
    }

    println!("Answer: {}", answer);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Usage: cargo run [filename]");
    }

    // Get filepath from argument
    let filepath = String::from(&args[1]);

    // Read data from file
    let data = read_file_to_string(filepath);
    solve(data);
}
