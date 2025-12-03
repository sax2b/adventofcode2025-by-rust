use std::env;
use std::fs;

fn read_file_to_string(filepath: String) -> String {
    let contents = fs::read_to_string(filepath)
        .expect("Should have been able to read the file");

    return contents
}

fn solve(data: String) {
    let rotations: Vec<&str> = data.split("\n").collect();

    let mut dial:i32 = 50; // Dial starts by pointing at 50
    let mut password:i32 = 0;

    for r in rotations {
        println!("Rotation: {}", r);

        let (d, num_str)= r.split_at(1);

        let direction = d.chars().next().unwrap();
        let mut num: i32 = num_str.parse().unwrap();
    
        num = num%100;

        println!("\tDirection {} with value {}", direction, num);
        

        match direction {
            'L' => {
                dial -= num;
            },
            'R' => {
                dial += num;
            },
            _ => panic!("\tUnknown direction: {}", direction)
        }

        if dial < 0 {
            dial += 100;
        } else if dial >= 100 {
            dial -= 100;
        }

        if dial == 0 {
            password += 1;
        }

        println!("\tCurrent dial: {}", dial);
        
    }

    println!("Answer: {}", password);
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
