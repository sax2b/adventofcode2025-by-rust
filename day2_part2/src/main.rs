use std::array::repeat;
use std::env;
use std::fs;

fn read_file_to_string(filepath: String) -> String {
    let contents = fs::read_to_string(filepath)
        .expect("Should have been able to read the file");

    return contents
}

fn is_repeated_at_least_twice(num: i64) -> bool {
    let num_str = num.to_string();

    if !num_str.chars().all(|c| c.is_numeric()) {
        return false;
    }

    let size = num_str.len();

    for i in 1..size {
        let repeated_size = i ;

        if size%repeated_size != 0 {
            continue;
        }

        let mut position = 0;
        let mut found = true;
        while position < size {

            let a = position;
            let b = a+repeated_size;
            let c = b+repeated_size;

            if num_str[a..b] != num_str[b..c] {
                found = false;
                break;
            }

            if c == size {
                break;
            }

            position += repeated_size;
        }

        if found {
            return true
        }
    }

    return false
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
            // println!("Number {} {}", number, is_repeated_at_least_twice(number));
            if is_repeated_at_least_twice(number) {
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
