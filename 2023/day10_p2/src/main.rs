use std::{
    collections::VecDeque,
    fs::File,
    io::{BufRead, BufReader},
    usize,
};

use array_tool::vec::{Intersect, Union};

fn read_file_line_by_line(
    filepath: &str,
    data: &mut Vec<char>,
) -> Result<(usize, usize), Box<dyn std::error::Error>> {
    let file = File::open(filepath)?;
    let reader = BufReader::new(file);

    let mut width = 0;
    let mut height = 0;
    for line in reader.lines() {
        let line = line.unwrap();

        let mut count = 0;
        for (_, c) in line.chars().enumerate() {
            data.push(c);
            count += 1;
        }
        if count > width {
            width = count;
        }
        height += 1;
    }

    Ok((height, width))
}

fn find_starting_position(data: &Vec<char>, height: usize, width: usize) -> (usize, usize) {
    for i in 0..height {
        for j in 0..width {
            if data[i * width + j] == 'S' {
                return (i, j);
            }
        }
    }

    panic!("No starting position found");
}

fn clean_grid(data: &mut Vec<char>, height: usize, width: usize, visited: &Vec<(usize, usize)>) {
    for i in 0..height {
        for j in 0..width {
            if !visited.contains(&(i, j)) {
                data[i * width + j] = '.';
            }
        }
    }
}

fn find_longest_path(
    data: &mut Vec<char>,
    height: usize,
    width: usize,
) -> (Vec<(usize, usize)>, u32) {
    let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
    let mut visited: Vec<(usize, usize)> = Vec::new();
    let mut maybe_s: Vec<char> = vec!['|', '-', '7', 'J', 'L', 'F'];

    let starting_position = find_starting_position(data, height, width);
    queue.push_back(starting_position);
    visited.push(starting_position);

    while !queue.is_empty() {
        let (y, x) = queue.pop_front().unwrap();
        let current = data[y * width + x];

        if y > 0
            && "S|JL".contains(current)
            && "|7F".contains(data[(y - 1) * width + x])
            && !visited.contains(&(y - 1, x))
        {
            queue.push_back((y - 1, x));
            visited.push((y - 1, x));
            if current == 'S' {
                maybe_s = maybe_s.intersect(vec!['|', 'J', 'L']);
            }
        }

        if y < height - 1
            && "S|7F".contains(current)
            && "|JL".contains(data[(y + 1) * width + x])
            && !visited.contains(&(y + 1, x))
        {
            queue.push_back((y + 1, x));
            visited.push((y + 1, x));
            if current == 'S' {
                maybe_s = maybe_s.intersect(vec!['|', '7', 'F']);
            }
        }

        if x > 0
            && "S-7J".contains(current)
            && "-LF".contains(data[y * width + x - 1])
            && !visited.contains(&(y, x - 1))
        {
            queue.push_back((y, x - 1));
            visited.push((y, x - 1));
            if current == 'S' {
                maybe_s = maybe_s.intersect(vec!['-', '7', 'J']);
            }
        }

        if x < width - 1
            && "S-LF".contains(current)
            && "-J7".contains(data[y * width + x + 1])
            && !visited.contains(&(y, x + 1))
        {
            queue.push_back((y, x + 1));
            visited.push((y, x + 1));
            if current == 'S' {
                maybe_s = maybe_s.intersect(vec!['-', 'L', 'F']);
            }
        }
    }

    println!("Starting position: {:?}", starting_position);
    if maybe_s.len() == 1 {
        let (y, x) = starting_position;
        data[y * width + x] = maybe_s[0];
    } else {
        println!("S is: {}", maybe_s[0]);
        panic!("S is not unique or S not found");
    }

    clean_grid(data, height, width, &visited);
    let length = visited.len() as u32;

    (visited, length / 2)
}

fn calculate_inside_fields(
    data: &mut Vec<char>,
    height: usize,
    width: usize,
    visited: &Vec<(usize, usize)>,
) -> u32 {
    let mut outside: Vec<(usize, usize)> = Vec::new();

    for y in 0..height {
        let mut within = false;
        let mut up: Option<bool> = None;
        for x in 0..width {
            let current = data[y * width + x];
            if current == '|' {
                assert!(up == None);
                within = !within;
            } else if current == '-' {
                assert!(up != None);
            } else if "LF".contains(current) {
                assert!(up == None);

                up = Some(current == 'L');
            } else if "7J".contains(current) {
                assert!(up != None);
                let mut expected = '7';
                if up == Some(true) {
                    expected = 'J';
                }
                if current != expected {
                    within = !within;
                }
                up = None;
            } else if current == '.' {
            } else {
                panic!("Unexpected character (horizontal): {}", current);
            }
            if !within {
                outside.push((y, x));
            }
        }
    }

    outside = outside.union(visited.to_vec());
    let inside_fields = width * height - outside.len();

    inside_fields as u32
}

fn main() {
    let mut data: Vec<char> = Vec::new();
    let Ok((height, width)) = read_file_line_by_line("src/day10_input", &mut data) else {
        println!("Error reading file");
        return;
    };

    println!("Height: {}, Width: {}", height, width);

    let (visited, longest_path) = find_longest_path(&mut data, height, width);
    println!("Longest path: {}", longest_path);

    for i in 0..height {
        for j in 0..width {
            print!("{}", data[i * width + j]);
        }
        println!();
    }

    let inside_fields = calculate_inside_fields(&mut data, height, width, &visited);
    println!("Inside fields: {}", inside_fields);
}
