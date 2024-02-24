use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;

fn get_number_word_vec() -> Vec<&'static str> {
    let mut number_strings: Vec<&str> = Vec::new();
    number_strings.push("zero");
    number_strings.push("one");
    number_strings.push("two");
    number_strings.push("three");
    number_strings.push("four");
    number_strings.push("five");
    number_strings.push("six");
    number_strings.push("seven");
    number_strings.push("eight");
    number_strings.push("nine");

    number_strings
}

fn find_first_number(line: &str) -> char {
    let number_strings = get_number_word_vec();
    let result = line.find(|x: char| x.is_numeric());

    let mut index: usize = usize::MAX;
    let mut num_string_index: usize = usize::MAX;
    let mut is_word_number = false;

    if !result.is_none() {
        index = result.unwrap();
    }

    for (i,filter) in number_strings.iter().enumerate() {
        let pos = line.find(filter).unwrap_or(usize::MAX);
        if pos < index {
            index = pos;
            num_string_index = i;
            is_word_number = true;
        }
    }

    let number: char;
    if is_word_number {
        number = num_string_index.to_string().chars().nth(0).unwrap();
    } else {
        number = line.chars().nth(index).unwrap();
    }

    number
}

fn find_last_number(line: &str) -> char {
    let number_strings = get_number_word_vec();
    let result = line.rfind(|x: char| x.is_numeric());

    let mut index: usize = 0;
    let mut num_string_index: usize = usize::MAX;
    let mut is_word_number = false;

    if !result.is_none() {
        index = result.unwrap();
    }

    for (i,filter) in number_strings.iter().enumerate() {
        let pos = line.rfind(filter).unwrap_or(usize::MAX);

        if pos > index && pos < usize::MAX {
            index = pos;
            num_string_index = i;
            is_word_number = true;
        }
    }

    let number: char;
    if is_word_number {
        number = num_string_index.to_string().chars().nth(0).unwrap();
    } else {
        number = line.chars().nth(index).unwrap();
    }

    number
}

fn find_numbers(line: &str) -> u32 {
    let first_number = find_first_number(&line);
    let last_number = find_last_number(&line);

    (format!("{}{}", first_number, last_number)).parse::<u32>().unwrap()
}

fn read_file_line_by_line(filepath: &str) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open(filepath)?;
    let reader = BufReader::new(file);
    let mut total: u32 = 0;

    for line in reader.lines() {
        let line = line.unwrap();
        let number = find_numbers(&line);
        total += number;
        println!("Line: {} / Number: {}", line, number);
    }

    println!("----------");
    println!("Total: {}", total);

    Ok(())
}

fn main() {
    let _ = read_file_line_by_line("src/day1/input_d1p1");
    // let _ = read_file_line_by_line("src/day1/input_file_with_words");
}
