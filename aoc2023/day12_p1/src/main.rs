use ::regex::Regex;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

mod variations;
mod regex;

#[derive(Debug)]
struct Row {
    data: Vec<char>,
    sequence: Vec<u32>,
}

impl Row {
    fn new(data: Vec<char>, sequence: Vec<u32>) -> Self {
        Row { data, sequence }
    }

    pub fn possibilities(&self) -> u32 {
        let mut count = 0;

        let possibilities = self.generate_possibilites();
        let formatted = regex::generate_regex(&self.sequence);
        let regex = Regex::new(&formatted).unwrap();

        for possibility in possibilities {
            if regex.is_match(&possibility) {
                count += 1;
            }
        }

        count
    }

    fn generate_possibilites(&self) -> Vec<String> {
        let mut possibilities: Vec<String> = Vec::new();

        let variation_base = String::from_iter(self.data.iter());

        let mut count: u32 = 0;
        let different_chars: u32 = 2;
        for char in self.data.iter() {
            if *char == '?' {
                count += 1;
            }
        }

        let num_variations: usize = different_chars.pow(count) as usize;
        let variations =
            variations::generate_variations(count as usize, num_variations, vec!['#', '.']);

        for variation in variations.iter() {
            let mut current_variation = variation_base.clone();

            for char in variation.chars() {
                current_variation = current_variation.replacen("?", &char.to_string(), 1);
            }

            possibilities.push(current_variation);
        }

        possibilities
    }
}

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

        let sequence = parts[1]
            .split(",")
            .map(|x| x.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();
        data.push(Row::new(parts[0].chars().collect::<Vec<char>>(), sequence));
    }

    Ok(())
}

fn possibilities(data: &Vec<Row>) -> u32 {
    let mut count = 0;

    for row in data.iter() {
        count += row.possibilities();
    }

    count
}

fn main() {
    let mut data: Vec<Row> = Vec::new();

    let _ = read_file_line_by_line("src/day12_input", &mut data);

    let count = possibilities(&data);
    println!("Count: {}", count);
}
