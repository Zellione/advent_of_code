use row::Row;
use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
    time::Instant,
};

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
        let data_part = format!("{}?{}?{}?{}?{}", data_part, data_part, data_part, data_part, data_part);

        let mut sequence = parts[1]
            .split(",")
            .map(|x| x.parse::<u64>().unwrap())
            .collect::<Vec<u64>>();
        let base_sequence = sequence.clone();
        for _ in 0..4 {
            sequence.append(base_sequence.clone().as_mut());
        }

        data.push(Row::new(data_part, sequence));
    }

    Ok(())
}

fn trim_start(pattern: &str) -> &str {
    if pattern.starts_with(".") {
        return &pattern[1..];
    }

    &pattern
}

fn find_possibilities(
    cache: &mut HashMap<String, u64>,
    pattern: String,
    sequence: Vec<u64>,
) -> u64 {
    let line = format!(
        "{} {}",
        pattern,
        sequence.iter().map(|x| x.to_string()).collect::<String>()
    );

    if cache.contains_key(&line) {
        return *cache.get(&line).unwrap();
    }

    if sequence.len() <= 0 {
        if !pattern.contains("#") {
            return 1;
        }

        return 0;
    }

    if (pattern.len() as i32)
        - (sequence.iter().copied().reduce(|a, b| a + b).unwrap() as i32)
        - (sequence.len() as i32)
        + 1
        < 0
    {
        return 0;
    }

    let damaged_or_unknown = !&pattern[0..sequence[0] as usize].contains(".");

    if pattern.len() == sequence[0] as usize {
        if damaged_or_unknown {
            return 1;
        }

        return 0;
    }

    if !cache.contains_key(&line) {
        let sequence_copy = sequence.clone();
        let mut part1 = 0;
        if pattern.as_bytes()[0] as char != '#' {
            part1 = find_possibilities(cache, trim_start(&pattern[1..]).to_string(), sequence);
        }

        let mut part2 = 0;
        if damaged_or_unknown && pattern.as_bytes()[sequence_copy[0] as usize] as char != '#' {
            part2 = find_possibilities(
                cache,
                trim_start(&pattern[(sequence_copy[0] as usize + 1)..]).to_string(),
                sequence_copy[1..].to_vec(),
            );
        }

        cache.insert(line.clone(), part1 + part2);
    }

    *cache.get(&line).unwrap()
}

fn possibilities(data: &Vec<Row>) -> u64 {
    let mut cache: HashMap<String, u64> = HashMap::new();

    let data = data.clone();
    let mut count: u64 = 0;

    for row in data {
        count += find_possibilities(&mut cache, row.data, row.sequence);
    }

    count
}

fn main() {
    let mut data: Vec<Row> = Vec::new();
    let now = Instant::now();

    let _ = read_file_line_by_line("src/day12_input", &mut data);

    let count = possibilities(&data);
    let elapsed = now.elapsed();

    println!("Count: {} / Time elapsed: {:?}", count, elapsed);
}
