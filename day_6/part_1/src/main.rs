use std::{fs::File, io::{BufRead, BufReader, Result}};

fn read_file(file_name: &str) -> Result<Vec<String>> {
    let input_file = File::open(file_name)
        .expect("couldn't read file");
    let buf = BufReader::new(input_file);
    return buf.lines().collect();
}

fn main() {
    let mut lines = read_file("src/input.txt").unwrap();

    let mut products: Vec<i64> = Vec::new();
    let mut operations: Vec<String> = Vec::new();

    lines.reverse();
    for line in &lines {
        if line.is_empty() {
            break;
        }

        if line.contains("+") || line.contains("*") {
            println!("operations");
            for op in line.split_whitespace() {
                operations.push(op.to_string());
            }
            continue;
        }

        println!("{line}");

        let mut i: usize = 0;
        for num in line.split_whitespace() {
            println!("num: {num}");
            let num: i64 = num.parse().unwrap();
            if i >= products.len() {
                // first row
                products.push(num);
            } else {
                if operations[i] == "+" {
                    products[i] += num;
                } else {
                    products[i] *= num;
                }
            }
            i += 1;
        }
    }
    println!("products: {:?}", products);

    let mut sum = 0;
    for product in products {
        sum += product;
    }
    println!("sum: {sum}");
}
