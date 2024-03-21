use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn read_file_line_by_line(
    filepath: &str,
    data: &mut Vec<char>,
) -> Result<(usize, usize), Box<dyn std::error::Error>> {
    let file = File::open(filepath)?;
    let reader = BufReader::new(file);

    let mut width = 0;
    let mut height = 0;
    let mut index = 1;
    for line in reader.lines() {
        let line = line.unwrap();

        let mut count = 0;
        for (_, c) in line.chars().enumerate() {
            if c == '#' {
                data.push(std::char::from_digit(index, 10).unwrap());
                index += 1;
            } else {
                data.push(c);
            }
            count += 1;
        }
        if count > width {
            width = count;
        }
        height += 1;
    }

    Ok((height, width))
}

fn add_row_after(data: &mut Vec<char>, height: usize, width: usize, row: usize) {
    let mut new_data: Vec<char> = Vec::new();
    for y in 0..height {
        for x in 0..width {
            new_data.push(data[y * width + x]);
        }
        if y == row {
            for _ in 0..width {
                new_data.push('.');
            }
        }
    }

    *data = new_data;
}

fn add_col_after(data: &mut Vec<char>, height: usize, width: usize, col: usize) {
    let mut new_data: Vec<char> = Vec::new();
    for y in 0..height {
        for x in 0..width {
            new_data.push(data[y * width + x]);
            if x == col {
                new_data.push('.');
            }
        }
    }

    *data = new_data;
}

fn expand_empty_rows_and_cols(
    data: &Vec<char>,
    height: usize,
    width: usize,
) -> (Vec<char>, usize, usize) {
    let mut grid: Vec<char> = data.clone();
    let mut new_height = height;
    let mut new_width = width;

    // Scan rows
    let mut delta = 0;
    'outer: for y in 0..height {
        for x in 0..width {
            if data[y * width + x] != '.' {
                continue 'outer;
            }
        }

        // Empty row
        add_row_after(&mut grid, new_height + delta, new_width, y + delta);
        delta += 1;
    }
    new_height += delta;

    // Scan columns
    let mut delta: usize = 0;
    'outer: for x in 0..width {
        for y in 0..height {
            if data[y * width + x] != '.' {
                continue 'outer;
            }
        }

        // Empty column
        add_col_after(&mut grid, new_height, new_width + delta, x + delta);
        delta += 1;
    }
    new_width += delta;

    (grid, new_height, new_width)
}

fn main() {
    let mut data: Vec<char> = Vec::new();
    let Ok((height, width)) = read_file_line_by_line("src/day11_calib", &mut data) else {
        println!("Error reading file");
        return;
    };
    let (data, height, width) = expand_empty_rows_and_cols(&data, height, width);

    for y in 0..height {
        for x in 0..width {
            print!("{}", data[y * width + x]);
        }
        println!();
    }
}
