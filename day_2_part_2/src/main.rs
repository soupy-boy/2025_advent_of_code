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

            // loop through numbers in range
            for num in lower_bound..upper_bound + 1 {
                let num_str = num.to_string();
                let num_len = num_str.len();
                if num_len < 2 {
                    continue;
                }

                // loop through all ways of slicing num
                let mut invalid = false;
                'slices: for num_repeats in 2..=num_len {
                    if num_len % num_repeats != 0 {
                        continue;
                    }
                    // num_repeats goes evenly into num
                    // check if every subsection is the same
                    let sub_size = num_str.len() / num_repeats;
                    let sub_num = &num_str[0..sub_size];
                    // loop through every sub string in num
                    for i in 1..num_repeats {
                        if &num_str[(i * sub_size)..((i * sub_size) + sub_size)] != sub_num {
                            continue 'slices;
                        }
                    }
                    invalid = true;
                }
                if invalid {
                    sum += num;
                    println!("num is invalid");
                    println!("num: {num}");
                }
            }

        }
    }
    println!("sum: {sum}");
}
