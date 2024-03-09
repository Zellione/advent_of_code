use std::{
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug)]
struct Race {
    time: u64,
    distance: u64,

    possibilities: u64,
}

impl Race {
    pub fn new(time: u64, distance: u64) -> Self {
        Race {
            time,
            distance,
            possibilities: 0,
        }
    }

    fn count_possibilities(&mut self) {
        let mut ms_to_push = 0;
        while ms_to_push <= self.time {
            let time_left = self.time - ms_to_push;
            let distance = ms_to_push * time_left;
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

    let mut times: Vec<String> = Vec::new();
    let mut distances: Vec<String> = Vec::new();
    for line in reader.lines() {
        let line = line.expect("to have content");
        if line.contains("Time") {
            times = line
                .split(':')
                .last()
                .expect("to have numbers")
                .split(" ")
                .filter(|n| n.trim() != "")
                .map(|n| n.trim().to_string())
                .collect();
        }

        if line.contains("Distance") {
            distances = line
                .split(':')
                .last()
                .expect("to have numbers")
                .split(" ")
                .filter(|n| n.trim() != "")
                .map(|n| n.trim().to_string())
                .collect();
        }
    }

    let time = times.join("").parse::<u64>().expect("to be a number");
    let distance = distances.join("").parse::<u64>().expect("to be a number");
    println!("{time} {distance}");
    races.push(Race::new(time, distance));

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
