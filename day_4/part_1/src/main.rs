use std::{fs::File, io::{BufRead, BufReader, Result}};

fn read_file(file_name: &str) -> Result<Vec<String>> {
    let input_file = File::open(file_name)
        .expect("couldn't read file");
    let buf = BufReader::new(input_file);
    return buf.lines().collect();
}

fn grid_val(grid: &Vec<Vec<u8>>, x: i64, y: i64) -> u8 {
    if x < 0 || x >= grid.len() as i64 || y < 0 || y >= grid[0].len() as i64 {
        // out of bounds
        return 0;
    }
   return grid[x as usize][y as usize];
}

fn adjacent_under_four(grid: &Vec<Vec<u8>>, x: i64, y: i64) -> bool {
    let val = grid_val(&grid, x - 1, y - 1) + grid_val(&grid, x, y - 1) + grid_val(&grid, x + 1, y - 1) +
              grid_val(&grid, x - 1, y) + grid_val(&grid, x + 1, y) + 
              grid_val(&grid, x - 1, y + 1) + grid_val(&grid, x, y + 1) + grid_val(&grid, x + 1, y + 1);
    println!("grid[{x}][{y}]: {val}");
    if val < 4 {
        println!("under 4");
        return true;
    } else {
        return false;
    }
}

fn main() {
    let mut accessible: u64 = 0;
    let lines = read_file("src/input.txt").unwrap();

    let width = lines.first().unwrap().len();
    let length = lines.len();

    println!("width: {width}");
    println!("length: {length}");

    let mut grid: Vec<Vec<u8>> = Vec::new();
    for line in &lines {
        if line.is_empty() {
            continue;
        }

        println!("{line}");

        let int_line: Vec<u8> = line.chars().into_iter().map(|c| if c == '.' {0} else {1})
            .collect();
        println!("int_array: {:?}", int_line);

        grid.push(int_line);
        println!("grid: {:?}", grid);
    }

    for (row_index, row) in grid.iter().enumerate() {
        for (column_index, val) in row.iter().enumerate() {
            // check adjacent 8 positions, count if they add up to 4
            // if so increment accessible
            if val == &1u8 && adjacent_under_four(&grid, row_index as i64, column_index as i64) {
                accessible += 1;
            }
        }
    }
    println!("accessible: {accessible}");
}
