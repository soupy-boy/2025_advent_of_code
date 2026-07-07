use std::{fs::File, io::{BufRead, BufReader, Result}};

fn read_file(file_name: &str) -> Result<Vec<String>> {
    let input_file = File::open(file_name)
        .expect("couldn't read file");
    let buf = BufReader::new(input_file);
    return buf.lines().collect();
}

fn main() {
    let mut sum: u64 = 0;
    let lines = read_file("src/input.txt").unwrap();
    for line in &lines {
        if line.is_empty() {
            break;
        }

        println!("{line}");
        let product_ranges: Vec<&str> = line.split(",").collect();
        for product_range in product_ranges {
            println!("product_range: {product_range}");
            let product_range: Vec<&str> = product_range.split("-").collect();
            println!("product_range[0]: {}", product_range[0]);
            println!("product_range[1]: {}", product_range[1]);
            let lower_bound: u64 = product_range[0].parse().unwrap();
            let upper_bound: u64 = product_range[1].parse().unwrap();

            for num in lower_bound..upper_bound + 1 {
                let num_str = num.to_string();
                if num_str.len() < 2 || num_str.len() % 2 == 1 {
                    continue;
                }
                if num_str[0..num_str.len() / 2] == num_str[num_str.len() / 2..num_str.len()] {
                    sum += num;
                    println!("num is invalid");
                }
                println!("num: {num}");

            }

        }
    }
    println!("sum: {sum}");
}
