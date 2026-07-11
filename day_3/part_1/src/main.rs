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
        
        // to get the largest one that you should use for left battery, go from left to right 
        // keep track of largest found battery until you hit the very right. if a battery is 
        // the same size as the last found biggest one don't move to mark this as the biggest
        // keep the leftmost one (leaves more room for right batteries)
        
        // now going from very right to left keep track of largest right battery. once you meet
        // where the left one was, return the result of that bank.
        let left_ind = 0;
        let right_ind = line.len() - 1;

        let chars: Vec<char> = line.chars().collect();

        let mut largest_from_left: u8 = chars[left_ind].to_string().parse().unwrap();
        let mut largest_from_left_index: usize = left_ind;

        let mut largest_from_right: u8 = chars[right_ind].to_string().parse().unwrap();
        let mut largest_from_right_index: usize = right_ind;

        // move left pointer from very left to all but the very right digit (left battery can't be
        // that one)
        for i in 1..chars.len() - 1 {
            let i_num = chars[i].to_string().parse().unwrap();
            if i_num > largest_from_left {
                largest_from_left = i_num;
                largest_from_left_index = i;
            }

            // 9 is largest digit 
            if i_num == 9 {
                break;
            }
        }
        if largest_from_left_index == largest_from_right_index - 1 {
            let mut full_num = String::new();
            full_num.push(chars[largest_from_left_index]);
            full_num.push(chars[largest_from_right_index]);
            let full_num: u64 = full_num.parse().unwrap();
            println!("full_num: {full_num}");
            println!("largest_from_left_index: {largest_from_left_index}");
            println!("largest_from_right_index: {largest_from_right_index}");
            sum += full_num;
            continue;
        }
        // move right pointer from very right to 1 to the right of left
        for i in (largest_from_left_index + 1 .. largest_from_right_index).rev() {
            let i_num = chars[i].to_string().parse().unwrap();
            if i_num > largest_from_right {
                largest_from_right = i_num;
                largest_from_right_index = i;
            }

            // 9 is largest digit 
            if i_num == 9 {
                break;
            }
        }
        let mut full_num = String::new();
        full_num.push(chars[largest_from_left_index]);
        full_num.push(chars[largest_from_right_index]);
        let full_num: u64 = full_num.parse().unwrap();
        println!("full_num: {full_num}");
        println!("largest_from_left_index: {largest_from_left_index}");
        println!("largest_from_right_index: {largest_from_right_index}");
        sum += full_num;

    }
    println!("sum: {sum}");
}
