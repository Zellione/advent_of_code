use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;

fn print_array(pattern: &[char; num_rows() * num_columns()]) {
    for i in 0..num_rows() {
        for j in 0..num_columns() {
            print!("{}", pattern[i * num_rows() + j]);
        }
        print!("\n");
    }
}

fn read_file_line_by_line(filepath: &str) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open(filepath)?;
    let reader = BufReader::new(file);
    let mut input: [char; num_columns() * num_rows()] = ['.'; num_columns() * num_rows()];
    let mut index: usize = 0;

    for line in reader.lines() {
        let mut inner_index: usize = 0;
        let line = line.unwrap();
        for character in line.chars() {
            input[num_columns() * index + inner_index] = character;
            inner_index += 1;
        }

        index += 1;
    }

    print_array(&input);
    iterate_nums(&input);
    Ok(())
}

fn is_num_valid(pattern: &[char; num_rows() * num_columns()], pos_row: usize, pos_col: usize, width: usize) -> bool {
    let mut pos_row_low = pos_row;
    if pos_row_low != 0 {
        pos_row_low -= 1;
    }

    let mut pos_row_high = pos_row;
    if pos_row_high <= pattern.len() - 1 {
        pos_row_high += 2;
    }

    let mut pos_col_low = pos_col;
    if pos_col_low != 0 {
        pos_col_low -= 1;
    }

    let mut pos_col_high = pos_col;
    if pos_col_high + width <= num_columns() - 1 {
        pos_col_high += width + 1;
    }

    for row in pos_row_low..pos_row_high {
        for col in pos_col_low..pos_col_high {
            let current_char = pattern[row * num_rows() + col];

            if !current_char.is_numeric() && current_char != '.' {
                return true;
            }
        }
    }

    false
}

fn iterate_nums(pattern: &[char; num_rows() * num_columns()]) {
    let mut sum: u32 = 0;

    for i_row in 0..num_rows() {
        let mut number: String = String::new();

        for i_col in 0..num_columns() {
            let cur_index = num_rows() * i_row + i_col;
            if pattern[cur_index].is_numeric() {
                number.push(pattern[cur_index]);
            }

            if !number.is_empty() && !pattern[cur_index].is_numeric() {
                let num = number.parse::<u32>().unwrap();

                if is_num_valid(pattern, i_row, i_col - number.len(), number.len()) {
                    sum += num;

                    println!("Found valid number: {}", num);
                }

                number = String::new();
            }
        }
    }

    println!("-------------------------------");
    println!("The sum of valid numbers is: {}", sum);
}

const fn num_rows() -> usize {
    const ROWS: usize = 141;

    ROWS
}

const fn num_columns() -> usize {
    const COLUMNS: usize = 141;

    COLUMNS
}


fn main() {
    let _ = read_file_line_by_line("src/day3/day3_input");
}
