use std::env;
use std::fs;

fn read_file_to_string(filepath: String) -> String {
    let contents = fs::read_to_string(filepath)
        .expect("Should have been able to read the file");

    return contents
}

fn solve(data: String) {
    let mut answer: i64 = 0;

    let banks: Vec<&str> = data.split("\n").collect();
    for bank in banks {

        let bank = bank.to_string();

        let mut max_joltage: i64 = 0;
        for i in 0..bank.len() {
            // println!("\ti: {}", &bank[i..i+1]);
            for j in i+1..bank.len() {
                // println!("\tj: {}", &bank[j..j+1]);
                let joltage = format!("{}{}", &bank[i..i+1], &bank[j..j+1]).parse().unwrap();
                if joltage > max_joltage {
                    max_joltage = joltage;
                }
            }
        }

        println!("Bank: {} Max Joltage: {}", bank, max_joltage);

        answer += max_joltage;
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
