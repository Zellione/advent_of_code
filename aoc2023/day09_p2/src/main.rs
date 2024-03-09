use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn read_file_line_by_line(filepath: &str) -> Result<Vec<Vec<i32>>, Box<dyn std::error::Error>> {
    let file = File::open(filepath)?;
    let reader = BufReader::new(file);
    let mut numbers: Vec<Vec<i32>> = Vec::new();

    for line in reader.lines() {
        let mut numbers_line: Vec<i32> = Vec::new();
        let line = line.unwrap();
        for number in line.split_whitespace() {
            numbers_line.push(number.parse().unwrap());
        }
        numbers.push(numbers_line);
    }

    Ok(numbers)
}

fn find_sequence_continuation(numbers: &Vec<i32>) -> i32 {
    let mut continuation: Vec<Vec<i32>> = Vec::new();
    continuation.push(numbers.clone());

    loop {
        let mut new_continuation: Vec<i32> = Vec::new();
        let last = continuation.last().unwrap();
        let mut all_zeros = true;
        for i in 0..last.len() - 1 {
            let item = last[i];
            let next = last[i + 1];
            let sum = next - item;
            new_continuation.push(sum);
            if sum != 0 {
                all_zeros = false;
            }
        }

        continuation.push(new_continuation.clone());

        if all_zeros {
            break;
        }
    }

    let mut prev_value_to_apply = i32::MAX;
    while continuation.len() > 0 {
        let last = continuation.pop().unwrap();
        if prev_value_to_apply != i32::MAX {
            prev_value_to_apply = last[0] - prev_value_to_apply;
        } else {
            prev_value_to_apply = last[0];
        }
    }

    prev_value_to_apply
}

fn main() {
    let numbers = read_file_line_by_line("src/day9_input").unwrap();

    let mut sum = 0;
    for line in &numbers {
        sum += find_sequence_continuation(line);
    }

    println!("Sum: {}", sum);
}
