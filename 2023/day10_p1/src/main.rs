use std::{
    fs::File,
    io::{BufRead, BufReader},
    usize,
};

const WIDTH: usize = 140;
const HEIGHT: usize = 140;
const ARRAY_SIZE: usize = WIDTH * HEIGHT;

fn read_file_line_by_line(
    filepath: &str,
    data: &mut [char; ARRAY_SIZE],
) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open(filepath)?;
    let reader = BufReader::new(file);

    let mut j = 0;
    for line in reader.lines() {
        let line = line.unwrap();

        for (i, c) in line.chars().enumerate() {
            data[j * WIDTH + i] = c;
        }
        j += 1;
    }

    Ok(())
}

fn find_farthest_point(data: &mut [char; ARRAY_SIZE]) -> u32 {
    let mut farthest_point = 0;

    let (start_x, start_y) = find_starting_point(data);
    println!("Start point: ({}, {})", start_x, start_y);
    let mut next_points = find_next_points(data, start_x as isize, start_y as isize);
    loop {
        farthest_point += 1;
        let mut new_points: Vec<(usize, usize)> = Vec::new();
        for next_point in &next_points {
            new_points.extend(find_next_points(
                data,
                next_point.0 as isize,
                next_point.1 as isize,
            ));
        }

        if new_points.len() == 0 {
            break;
        }

        next_points = new_points;
    }

    farthest_point
}

fn find_next_points(
    data: &mut [char; ARRAY_SIZE],
    start_x: isize,
    start_y: isize,
) -> Vec<(usize, usize)> {
    let mut next_points: Vec<(usize, usize)> = Vec::new();

    let points_to_check = vec![
        ((start_x - 1).clamp(0, (WIDTH - 1) as isize), start_y),
        ((start_x + 1).clamp(0, (WIDTH - 1) as isize), start_y),
        (start_x, (start_y - 1).clamp(0, (HEIGHT - 1) as isize)),
        (start_x, (start_y + 1).clamp(0, (HEIGHT - 1) as isize)),
    ];
    let start_x = start_x as usize;
    let start_y = start_y as usize;

    for (i, j) in points_to_check {
        let pos = (i * WIDTH as isize + j) as usize;
        let i = i as usize;
        let j = j as usize;
        match data[pos] {
            '|' => {
                if start_x != i && start_y == j {
                    next_points.push((i, j));
                    data[pos] = '*';
                }
            }
            '-' => {
                if start_y != j && start_x == i {
                    next_points.push((i, j));
                    data[pos] = '*';
                }
            }
            'L' => {
                if start_x < i && start_y == j || start_y > j && start_x == i {
                    next_points.push((i, j));
                    data[pos] = '*';
                }
            }
            'J' => {
                if start_x < i && start_y == j || start_y < j && start_x == i {
                    next_points.push((i, j));
                    data[pos] = '*';
                }
            }
            '7' => {
                if start_x > i && start_y == j || start_y < j && start_x == i {
                    next_points.push((i, j));
                    data[pos] = '*';
                }
            }
            'F' => {
                if start_x > i && start_y == j || start_y > j && start_x == i {
                    next_points.push((i, j));
                    data[pos] = '*';
                }
            }
            _ => {}
        }
    }
    data[start_x * WIDTH + start_y] = '*';

    next_points
}

fn find_starting_point(data: &[char; ARRAY_SIZE]) -> (usize, usize) {
    for i in 0..HEIGHT {
        for j in 0..WIDTH {
            if data[i * WIDTH + j] == 'S' {
                return (i, j);
            }
        }
    }
    (0, 0)
}

fn main() {
    let mut data = ['.'; ARRAY_SIZE];
    let _ = read_file_line_by_line("src/day10_input", &mut data);

    let farthest_point = find_farthest_point(&mut data);
    println!("Farthest point: {}", farthest_point);
}
