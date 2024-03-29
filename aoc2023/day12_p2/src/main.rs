use row::Row;
use std::{
    fs::File,
    io::{BufRead, BufReader},
    time::Instant,
};

mod regex;
mod row;

fn read_file_line_by_line(
    filepath: &str,
    data: &mut Vec<Row>,
) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open(filepath)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.unwrap();
        let line = line.split_whitespace();
        let parts = line.collect::<Vec<&str>>();
        let data_part = parts[0].to_string();
        // let base = data_part.clone();
        // for _ in 0..5 {
        //     data_part.append(base.clone().as_mut());
        // }

        let sequence = parts[1]
            .split(",")
            .map(|x| x.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();
        // let base_sequence = sequence.clone();
        // for _ in 0..5 {
        //     sequence.append(base_sequence.clone().as_mut());
        // }

        data.push(Row::new(data_part, sequence));
    }

    Ok(())
}

fn find_possibilities(row: &Row, mut pattern: String) -> u32 {
    let mut count: u32 = 0;

    for i in 0..pattern.len() {
        if (pattern.as_bytes()[i] as char) != '?' {
            continue;
        }

        pattern.replace_range(i..i + 1, "#");
        count += find_possibilities(row, pattern.clone());

        pattern.replace_range(i..i + 1, ".");
        return count + find_possibilities(row, pattern.clone());
    }

    if row.is_valid(&pattern) {
        println!("Valid: {} {:?}", pattern, row.sequence);
        return 1;
    }

    0
}

fn possibilities(data: &mut Vec<Row>) -> u32 {
    let mut count = 0;
    for row in data {
        count += find_possibilities(row, row.data.clone());
    }
    count
}

fn main() {
    let mut data: Vec<Row> = Vec::new();
    let now = Instant::now();

    let _ = read_file_line_by_line("src/day12_input", &mut data);

    let count = possibilities(&mut data);
    let elapsed = now.elapsed();

    println!("Count: {} / Time elapsed: {:?}", count, elapsed);
}
