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
            continue;
        }

        println!("{line}");
        
        // to get the largest one that you should use for left battery, go from left to right 
        // keep track of largest found battery until you hit the very right. if a battery is 
        // the same size as the last found biggest one don't move to mark this as the biggest
        // keep the leftmost one (leaves more room for right batteries)
        
        // now going from very right to left keep track of largest right battery. once you meet
        // where the left one was, return the result of that bank.
        let mut removals_allowed = line.len() - 12;

        if removals_allowed <= 0 {
            let full_num: u64 = line.parse().unwrap();
            println!("full_num: {full_num}");
            sum += full_num;
            continue;
        }

        let mut stack: Vec<u64> = Vec::new();

        for c in line.chars() {
            let num: u64 = u64::from(c.to_digit(10).unwrap());
            while removals_allowed > 0 && !stack.is_empty() && stack.last().unwrap() < &num {
                stack.pop();
                removals_allowed -= 1;
            }
            stack.push(num);
        }

        stack.truncate(12);

        // turn vector of u64 into full number
        let mut full_num: u64 = 0;
        for elem in &stack {
            full_num *= 10;
            full_num += elem;
        }

        println!("full_num: {full_num}");
        sum += full_num;

    }
    println!("sum: {sum}");
}
