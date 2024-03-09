use std::{
    collections::BTreeMap,
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug)]
struct Map {
    sequence: Vec<char>,
    entries: BTreeMap<String, (String, String)>,
}

impl Map {
    pub fn new(reader: BufReader<File>) -> Self {
        let mut sequence: Vec<char> = Vec::new();
        let mut entries: BTreeMap<String, (String, String)> = BTreeMap::new();

        let mut entries_start = false;
        for line in reader.lines() {
            let line = line.unwrap();
            if line == "" {
                entries_start = true;
                continue;
            }

            if entries_start {
                let elements: Vec<&str> = line.split("=").collect();
                let key = elements.first().unwrap().trim().to_string();

                let re = regex::Regex::new(r"(((?P<value1>\w+),\s(?P<value2>\w+)))").unwrap();
                let Some(values) = re.captures(elements.last().unwrap().trim()) else {
                    panic!("no match");
                };
                entries.insert(
                    key,
                    (
                        values.name("value1").unwrap().as_str().to_string(),
                        values.name("value2").unwrap().as_str().to_string(),
                    ),
                );
            } else {
                for c in line.chars() {
                    sequence.push(c);
                }
            }
        }

        Map { sequence, entries }
    }

    fn find_starts(&self) -> Vec<String> {
        let mut starts: Vec<String> = Vec::new();
        for (key, _) in self.entries.iter() {
            if key.chars().last().expect("to have chars") == 'A' {
                starts.push(key.to_string());
            }
        }
        starts
    }

    fn reached_end(index: &String) -> bool {
        index.chars().last().expect("to have chars") == 'Z'
    }

    fn calc_lcm(&self, steps: &Vec<String>) -> u64 {
        let mut lcm_num: u64 = 1;
        let mut steps_done: u64 = 0;
        let mut index: usize = 0;
        let mut lcms: Vec<u64> = Vec::new();

        for step in steps {
            let mut next_index = step;
            loop {
                if index >= self.sequence.len() {
                    index = 0;
                }

                if Map::reached_end(next_index) {
                    lcms.push(steps_done);
                    break;
                }

                let (left, right) = self.entries.get(next_index).expect("to have a start");
                let instruction = self.sequence.get(index).unwrap();
                match instruction {
                    'R' => {
                        next_index = right;
                    }
                    'L' => {
                        next_index = left;
                    }
                    _ => panic!("unknown instruction"),
                }

                steps_done += 1;
                index += 1;
            }
            steps_done = 0;
            index = 0;
        }

        for n in &lcms {
            lcm_num = num::integer::lcm(lcm_num, *n);
        }

        lcm_num
    }

    fn calc_steps(&self) -> u64 {
        let next_indices: Vec<String> = self.find_starts();
        self.calc_lcm(&next_indices)
    }
}

fn read_file_line_by_line(filepath: &str) -> Result<Map, Box<dyn std::error::Error>> {
    let file = File::open(filepath)?;
    let reader = BufReader::new(file);

    Ok(Map::new(reader))
}

fn main() {
    let map = read_file_line_by_line("src/day8_input").unwrap();
    let steps = map.calc_steps();
    println!("Number of steps: {}", steps);
}
