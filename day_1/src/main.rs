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
    let mut zero_passes: i16 = 0;
    let mut current_pos: i16 = 50;
    let lines = read_file("src/input.txt").unwrap();
    for line in &lines {

        if line.starts_with('L') {
            let amount = &line[get_byte_offset(&line, 1)..];
            let amount = amount.parse::<i16>().expect("failed to parse to int");

            // calculate how many times we passed zero
            if amount >= current_pos {
                if current_pos != 0 {
                    zero_passes += 1;
                }
                zero_passes += (amount - current_pos) / 100;
            }

            current_pos = (current_pos - amount).rem_euclid(100);
        } else if line.starts_with('R') {
            let amount = &line[get_byte_offset(&line, 1)..];
            let amount = amount.parse::<i16>().expect("failed to parse to int");

            // calculate how many times we passed zero
            zero_passes += (current_pos + amount) / 100;

            current_pos += amount;
            current_pos = current_pos % 100;
        }
        if current_pos == 0 {
            zero_occurences += 1;
        }
    }

    println!("zero_occurences: {}", zero_occurences);
    println!("zero_passes: {}", zero_passes);
}
