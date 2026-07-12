use std::{fs::File, io::{BufRead, BufReader, Result}};

fn read_file(file_name: &str) -> Result<Vec<String>> {
    let input_file = File::open(file_name)
        .expect("couldn't read file");
    let buf = BufReader::new(input_file);
    return buf.lines().collect();
}

fn main() {
    let lines = read_file("src/input.txt").unwrap();

    let mut ranges: Vec<(u64, u64)> = Vec::new();
    let mut num_fresh: u64 = 0;

    let mut at_ingredients = false;
    for line in &lines {
        if line.is_empty() {
            at_ingredients = true;
            continue;
        }

        println!("{line}");
        
        if !at_ingredients {
            // range
            let parts: Vec<&str> = line.split('-').collect();
            if parts.len() != 2 {
                println!("not a proper range");
                std::process::exit(0x0100);
            }
            let lower_range: u64 = parts[0].parse().unwrap();
            let upper_range: u64 = parts[1].parse().unwrap();
            ranges.push((lower_range, upper_range));
        } else {
            // ingredient
            let ingredient: u64 = line.parse().unwrap();

            for &(lower_bound, upper_bound) in &ranges {
                if ingredient >= lower_bound && ingredient <= upper_bound {
                    num_fresh += 1;
                    break;
                }
            }
        }
    }
    println!("num_fresh: {num_fresh}");
}
