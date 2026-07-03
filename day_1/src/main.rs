use std::{fs::File, io::{BufRead, BufReader, Result}};

fn read_file(file_name: &str) -> Result<Vec<String>> {
    let input_file = File::open(file_name)
        .expect("couldn't read file");
    let buf = BufReader::new(input_file);
    return buf.lines().collect();
}

fn get_byte_offset(text: &str, offset: usize) -> usize {
    let byte_offset = text
        .char_indices()
        .nth(offset)
        .map(|(i, _)| i)
        .unwrap_or(text.len());
    return byte_offset;
}

fn main() {
    let mut zero_occurences: u16 = 0;
    let mut zero_occurences_extra: i16 = 0;
    let mut current_pos: i16 = 50;
    let lines = read_file("input.txt").unwrap();
    for line in lines {
        if line.starts_with('L') {
            let amount = &line[get_byte_offset(&line, 1)..];
            let amount = amount.parse::<i16>().expect("failed to parse to int");
            let times_passed : i16 = (current_pos - amount).abs() / 100;
            println!("times_passed {}", times_passed);
            zero_occurences_extra += times_passed;

            current_pos = (current_pos - amount).rem_euclid(100);
        } else if line.starts_with('R') {
            let amount = &line[get_byte_offset(&line, 1)..];
            let amount = amount.parse::<i16>().expect("failed to parse to int");
            current_pos += amount;
            let times_passed : i16 = current_pos / 100;
            println!("times_passed {}", times_passed);
            zero_occurences_extra += times_passed;

            current_pos = current_pos % 100;
        }
        if current_pos == 0 {
            zero_occurences += 1;
        }
    }
    println!("zero_occurences: {}", zero_occurences);
    println!("zero_occurences_extra and normal: {}", zero_occurences_extra + zero_occurences_extra);
}
