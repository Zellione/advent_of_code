use std::{
    collections::BTreeMap,
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug, Clone, Copy)]
struct Point {
    x: usize,
    y: usize,

    empty_cols_before: usize,
    empty_rows_before: usize,
}

impl Point {
    fn new(x: usize, y: usize, empty_cols_before: usize, empty_rows_before: usize) -> Self {
        Point {
            x,
            y,
            empty_cols_before,
            empty_rows_before,
        }
    }

    fn distance(&self, other: &Point) -> usize {
        let multiplier = 1000000 - 1;
        let x = (self.x + self.empty_cols_before * multiplier) as isize - (other.x + other.empty_cols_before * multiplier) as isize;
        let y = (self.y + self.empty_rows_before * multiplier) as isize - (other.y + other.empty_rows_before * multiplier) as isize;

        (x.abs() + y.abs()) as usize
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

fn find_empty_rows_before_point(data: &Vec<char>, width: usize, point: Point) -> usize {
    let mut empty_rows: usize = 0;
    for y in 0..point.y {
        let mut empty = true;
        for x in 0..width {
            if data[y * width + x] != '.' {
                empty = false;
                break;
            }
        }

        if empty {
            empty_rows += 1;
        }
    }

    empty_rows
}

fn find_empty_cols_before_point(data: &Vec<char>, height: usize, width: usize, point: Point) -> usize {
    let mut empty_cols: usize = 0;
    for x in 0..point.x {
        let mut empty = true;
        for y in 0..height {
            if data[y * width + x] != '.' {
                empty = false;
                break;
            }
        }

        if empty {
            empty_cols += 1;
        }
    }

    empty_cols
}

fn find_points(data: &Vec<char>, height: usize, width: usize) -> BTreeMap<u32, Point> {
    let mut points: BTreeMap<u32, Point> = BTreeMap::new();

    let mut index = 1;
    for y in 0..height {
        for x in 0..width {
            if data[y * width + x] == '#' {
                let empty_cols_before = find_empty_cols_before_point(&data, height, width, Point::new(x, y, 0, 0));
                let empty_rows_before = find_empty_rows_before_point(&data, width, Point::new(x, y, 0, 0));
                points.insert(index, Point::new(x, y, empty_cols_before, empty_rows_before));
                index += 1;
            }
        }
    }

    points
}

fn calculate_shortest_paths(points: &BTreeMap<u32, Point>) -> usize {
    let mut sum: usize = 0;
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
    let Ok((height, width)) = read_file_line_by_line("src/day11_input", &mut data) else {
        println!("Error reading file");
        return;
    };
    let points = find_points(&data, height, width);
    let sum = calculate_shortest_paths(&points);

    println!("Sum: {}", sum);
}
