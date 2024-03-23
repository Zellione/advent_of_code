use std::{
    collections::BTreeMap,
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug, Clone, Copy)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn new(x: usize, y: usize) -> Self {
        Point { x, y }
    }

    fn distance(&self, other: &Point) -> u32 {
        let x = self.x as i32 - other.x as i32;
        let y = self.y as i32 - other.y as i32;

        (x.abs() + y.abs()) as u32
    }
}

fn read_file_line_by_line(
    filepath: &str,
    data: &mut Vec<char>,
) -> Result<(usize, usize), Box<dyn std::error::Error>> {
    let file = File::open(filepath)?;
    let reader = BufReader::new(file);

    let mut width: usize = 0;
    let mut height: usize = 0;
    for line in reader.lines() {
        let line = line.unwrap();

        let mut count: usize = 0;
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

fn add_row_after(data: &mut Vec<char>, height: usize, width: usize, row: usize) {
    let mut new_data: Vec<char> = Vec::new();
    for y in 0..height {
        for x in 0..width {
            new_data.push(data[y * width + x]);
        }
        if y == row {
            for _ in 0..width {
                new_data.push('.');
            }
        }
    }

    *data = new_data;
}

fn add_col_after(data: &mut Vec<char>, height: usize, width: usize, col: usize) {
    let mut new_data: Vec<char> = Vec::new();
    for y in 0..height {
        for x in 0..width {
            new_data.push(data[y * width + x]);
            if x == col {
                new_data.push('.');
            }
        }
    }

    *data = new_data;
}

fn expand_empty_rows_and_cols(
    data: &Vec<char>,
    height: usize,
    width: usize,
) -> (Vec<char>, usize, usize) {
    let mut grid: Vec<char> = data.clone();
    let mut new_height = height;
    let mut new_width = width;

    // Scan rows
    let mut delta = 0;
    'outer: for y in 0..height {
        for x in 0..width {
            if data[y * width + x] != '.' {
                continue 'outer;
            }
        }

        // Empty row
        add_row_after(&mut grid, new_height + delta, new_width, y + delta);
        delta += 1;
    }
    new_height += delta;

    // Scan columns
    let mut delta: usize = 0;
    'outer: for x in 0..width {
        for y in 0..height {
            if data[y * width + x] != '.' {
                continue 'outer;
            }
        }

        // Empty column
        add_col_after(&mut grid, new_height, new_width + delta, x + delta);
        delta += 1;
    }
    new_width += delta;

    (grid, new_height, new_width)
}

fn find_points(data: &Vec<char>, height: usize, width: usize) -> BTreeMap<u32, Point> {
    let mut points: BTreeMap<u32, Point> = BTreeMap::new();

    let mut index = 1;
    for y in 0..height {
        for x in 0..width {
            if data[y * width + x] == '#' {
                points.insert(index, Point::new(x, y));
                index += 1;
            }
        }
    }

    points
}

fn calculate_shortest_paths(points: &BTreeMap<u32, Point>) -> u32 {
    let mut sum: u32 = 0;
    for i in 1..points.len() + 1 {
        for j in i + 1..points.len() + 1 {
            let a = points.get(&(i as u32)).unwrap();
            let b = points.get(&(j as u32)).unwrap();
            sum += b.distance(a);
        }
    }

    sum
}

fn main() {
    let mut data: Vec<char> = Vec::new();
    let Ok((height, width)) = read_file_line_by_line("src/day11_calib", &mut data) else {
        println!("Error reading file");
        return;
    };
    let (data, height, width) = expand_empty_rows_and_cols(&data, height, width);
    let points = find_points(&data, height, width);
    let sum = calculate_shortest_paths(&points);

    println!("Sum: {}", sum);
}
