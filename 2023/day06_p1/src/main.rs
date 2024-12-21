use std::{
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug)]
struct Race {
    time: u32,
    distance: u32,

    possibilities: u32,
}

impl Race {
    pub fn new(time: u32, distance: u32) -> Self {
        Race {
            time,
            distance,
            possibilities: 0,
        }
    }

    fn count_possibilities(&mut self) {
        let mut ms_to_push = 0;
        while ms_to_push <= self.time {
            let mut distance = 0;
            for _ in 0 + ms_to_push..self.time {
                distance += ms_to_push;
            }
            if distance > self.distance {
                self.possibilities += 1;
            }
            ms_to_push += 1;
        }
    }
}

fn read_file_line_by_line(
    filepath: &str,
    races: &mut Vec<Race>,
) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open(filepath)?;
    let reader = BufReader::new(file);

    let mut times: Vec<u32> = Vec::new();
    let mut distances: Vec<u32> = Vec::new();
    for line in reader.lines() {
        let line = line.expect("to have content");
        if line.contains("Time") {
            times = line
                .split(':')
                .last()
                .expect("to have numbers")
                .split(" ")
                .filter(|n| n.trim() != "")
                .map(|n| n.trim())
                .map(|n| n.parse::<u32>().unwrap())
                .collect();
        }

        if line.contains("Distance") {
            distances = line
                .split(':')
                .last()
                .expect("to have numbers")
                .split(" ")
                .filter(|n| n.trim() != "")
                .map(|n| n.trim())
                .map(|n| n.parse::<u32>().unwrap())
                .collect();
        }
    }

    for i in 0..times.len() {
        let time = times.get(i).unwrap();
        let distance = distances.get(i).unwrap();
        races.push(Race::new(*time, *distance));
    }

    Ok(())
}

fn main() {
    let mut races: Vec<Race> = Vec::new();
    let _ = read_file_line_by_line("src/day6_input", &mut races);

    let mut result = 1;
    for race in &mut races {
        race.count_possibilities();
        println!(
            "Distance: {} Time: {} -> Found possibilities: {}",
            race.distance, race.time, race.possibilities
        );
        result *= race.possibilities;
    }

    println!("*********************************");
    println!("Multiplication of all results: {}", result);
}
