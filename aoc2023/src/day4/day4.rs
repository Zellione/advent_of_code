use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn read_file_line_by_line(filepath: &str) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open(filepath)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.expect("Should be a line");
        println!("{line}");
    }

    Ok(())
}

fn main() {
    println!("Welcome day 4");
    let _ = read_file_line_by_line("src/day4/day4_calib");
}
