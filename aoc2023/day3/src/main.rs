use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

struct CogAdjacent {
    cog_row: usize,
    cog_col: usize,
    number: u32,
}

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

fn is_num_valid(
    pattern: &[char; num_rows() * num_columns()],
    pos_row: usize,
    pos_col: usize,
    width: usize,
) -> bool {
    let mut pos_row_low = pos_row;
    if pos_row_low != 0 {
        pos_row_low -= 1;
    }

    let mut pos_row_high = pos_row;
    if pos_row_high <= num_rows() - 1 {
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

fn clamp_row(row: usize) -> (usize, usize) {
    let mut start_row: usize = row;
    let mut end_row: usize = row;

    if start_row > 0 {
        start_row -= 1;
    }

    if end_row < num_rows() - 1 {
        end_row += 2;
    }

    (start_row, end_row)
}

fn clamp_col(col: usize) -> (usize, usize) {
    let mut start_col: usize = col;
    let mut end_col: usize = col;

    if start_col > 0 {
        start_col -= 1;
    }

    if end_col < num_columns() - 1 {
        end_col += 2;
    }

    (start_col, end_col)
}

fn is_cog_adjacent(
    pattern: &[char; num_rows() * num_columns()],
    row: usize,
    col_start: usize,
    col_end: usize,
) -> (usize, usize, bool) {
    let (start_row, end_row) = clamp_row(row);
    let (start_col, _) = clamp_col(col_start);
    let (_, end_col) = clamp_col(col_end);

    for i_row in start_row..end_row {
        for i_col in start_col..end_col {
            let pos = i_row * num_rows() + i_col;
            if pattern[pos] == '*' {
                return (i_row, i_col, true);
            }
        }
    }

    (0, 0, false)
}

fn iterate_nums(pattern: &[char; num_rows() * num_columns()]) {
    let mut sum: u32 = 0;
    let mut gear_sum: u32 = 0;
    let mut gears: Vec<CogAdjacent> = Vec::new();

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

                    let (row_c, col_c, is_cog_adjacent) =
                        is_cog_adjacent(pattern, i_row, i_col - number.len(), i_col - 1);
                    if is_cog_adjacent {
                        let mut made_gear_multi = false;
                        for gear in &gears {
                            if gear.cog_col == col_c && gear.cog_row == row_c {
                                gear_sum += gear.number * num;
                                made_gear_multi = true;
                                break;
                            }
                        }

                        if !made_gear_multi {
                            let cog_adjacent = CogAdjacent {
                                cog_row: row_c,
                                cog_col: col_c,
                                number: num,
                            };
                            gears.push(cog_adjacent);
                        }
                    }
                }

                number = String::new();
            }
        }
    }

    println!("-------------------------------");
    println!("The sum of valid numbers is: {}", sum);
    println!("The sum of valid gear products is: {}", gear_sum);
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
    let _ = read_file_line_by_line("src/day3_input");
}
