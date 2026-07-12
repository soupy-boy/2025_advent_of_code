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

    for line in &lines {
        if line.is_empty() {
            break;
        }

        println!("{line}");
        
        // range
        let parts: Vec<&str> = line.split('-').collect();
        if parts.len() != 2 {
            println!("not a proper range");
            std::process::exit(0x0100);
        }
        let lower_range: u64 = parts[0].parse().unwrap();
        let upper_range: u64 = parts[1].parse().unwrap();
        ranges.push((lower_range, upper_range));
    }
    // merge ranges
    let mut changed = true;
    'changed_loop: while changed {
        changed = false;
        // determine if this goes into another range, if so modify that range
        for i in 0..ranges.len() {
            let (existing_lower, existing_upper) = ranges.get(i).unwrap();
            for j in 0..ranges.len() {
                if i == j {
                    continue
                }
                let (lower_range, upper_range) = ranges.get(j).unwrap();
                // cases
                // fully within existing range
                // lower lower range than existing, and higher upper range than existing lower and upper
                // lower lower range than existing, and upper range between existing lower and upper
                // higher upper range than existing, and lower range between existing lower and upper
                if lower_range >= existing_lower && upper_range <= existing_upper {
                    // fits fully within an existing range
                    changed = true;
                    ranges.remove(j);
                    continue 'changed_loop;
                } else if lower_range <= existing_lower && upper_range >= existing_upper {
                    // fully encompasses old range
                    changed = true;
                    ranges.remove(i);
                    continue 'changed_loop;
                } else if lower_range < existing_lower && upper_range <= existing_upper && upper_range <= existing_upper && upper_range >= existing_lower {
                    // newer only lower than existing
                    changed = true;
                    ranges[i] = (*lower_range, *existing_upper);
                    ranges.remove(j);
                    continue 'changed_loop;
                } else if lower_range >= existing_lower && upper_range > existing_upper && lower_range <= existing_upper && lower_range <= existing_upper{
                    // only higher
                    changed = true;
                    ranges[i] = (*existing_lower, *upper_range);
                    ranges.remove(j);
                    continue 'changed_loop;
                }
            }
        }
    }

    println!("{:?}", ranges);
    let mut num_fresh: u64 = 0;
    // now count how many numbers are encompassed by each range
    for (lower, upper) in ranges {
        num_fresh += upper - lower + 1;
    }
    println!("num_fresh: {num_fresh}");
}
