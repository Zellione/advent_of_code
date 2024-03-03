use core::{fmt, panic};
use std::{
    collections::BTreeMap,
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Eq, Hash, PartialEq, Debug, Ord, PartialOrd)]
enum MappingType {
    None,
    SeedToSoil,
    SoilToFertilizer,
    FertilizerToWater,
    WaterToLight,
    LightToTemperature,
    TemperatureToHumidity,
    HumidityToLocation,
}

impl fmt::Display for MappingType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            MappingType::None => write!(f, "None"),
            MappingType::SeedToSoil => write!(f, "SeedToSoil"),
            MappingType::SoilToFertilizer => write!(f, "SoilToFertilizer"),
            MappingType::FertilizerToWater => write!(f, "FertilizerToWater"),
            MappingType::WaterToLight => write!(f, "WaterToLight"),
            MappingType::LightToTemperature => write!(f, "LightToTemperature"),
            MappingType::TemperatureToHumidity => write!(f, "TemperatureToHumidity"),
            MappingType::HumidityToLocation => write!(f, "HumidityToLocation"),
        }
    }
}

struct Seed {
    number: u64,
    location: u64,
}

#[derive(Debug)]
struct Mapping {
    destination: u64,
    source: u64,
    range: u64,
}

impl Seed {
    pub fn new(seed: &str) -> Self {
        Seed {
            number: seed.parse().expect("this should be parseable as num"),
            location: 0,
        }
    }

    pub fn map_locations(&mut self, mappings: &BTreeMap<MappingType, Vec<Mapping>>) {
        let mut destination: u64 = self.number;

        for (_, mapping_step) in mappings {
            let mapping: Vec<&Mapping> = mapping_step
                .iter()
                .filter(|mapping| mapping.can_map(destination))
                .collect();
            let mapping = mapping.first();

            match mapping {
                None => destination = destination,
                Some(x) => destination = x.map_type(destination),
            }
        }
        self.location = destination;
    }
}

impl Mapping {
    pub fn new(line: &String) -> Self {
        let mapping: Vec<&str> = line
            .split(':')
            .last()
            .expect("should find mapping")
            .trim()
            .split(' ')
            .filter(|num| !num.trim().is_empty())
            .collect();

        assert_eq!(mapping.len(), 3);
        let destination = mapping
            .first()
            .expect("should have dest")
            .parse::<u64>()
            .expect("should be a number");
        let source = mapping
            .get(1)
            .expect("should have source")
            .parse::<u64>()
            .expect("should be a number");
        let range = mapping
            .last()
            .expect("should have range")
            .parse::<u64>()
            .expect("should be a number");
        Mapping {
            destination,
            source,
            range,
        }
    }

    pub fn map_type(&self, value: u64) -> u64 {
        for i in 0..self.range + 1 {
            let source = self.source + i;
            let dest = self.destination + i;

            if value == source {
                return dest;
            }
        }

        panic!("This should not happen")
    }

    pub fn can_map(&self, value: u64) -> bool {
        self.source <= value && self.source + self.range >= value
    }
}

fn make_seeds(line: &String) -> Vec<Seed> {
    let mut seeds_vec: Vec<Seed> = Vec::new();
    let seeds: Vec<&str> = line
        .split(':')
        .last()
        .expect("should find seeds")
        .trim()
        .split(' ')
        .filter(|num| !num.trim().is_empty())
        .collect();

    seeds.iter().for_each(|seed| {
        seeds_vec.push(Seed::new(&seed));
    });

    seeds_vec
}

fn read_file_line_by_line(filepath: &str) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open(filepath)?;
    let reader = BufReader::new(file);
    let mut seeds: Vec<Seed> = Vec::new();

    let mut mappings: BTreeMap<MappingType, Vec<Mapping>> = BTreeMap::new();

    let mut mapping_type: MappingType = MappingType::None;

    for line in reader.lines() {
        let line = line.expect("should be a line");
        if line.contains("seeds:") {
            seeds.extend(make_seeds(&line));
            continue;
        }
        if line.contains("seed-to-soil") {
            mapping_type = MappingType::SeedToSoil;
            mappings.insert(MappingType::SeedToSoil, Vec::new());
            continue;
        }
        if line.contains("soil-to-fertilizer") {
            mapping_type = MappingType::SoilToFertilizer;
            mappings.insert(MappingType::SoilToFertilizer, Vec::new());
            continue;
        }
        if line.contains("fertilizer-to-water") {
            mapping_type = MappingType::FertilizerToWater;
            mappings.insert(MappingType::FertilizerToWater, Vec::new());
            continue;
        }
        if line.contains("water-to-light") {
            mapping_type = MappingType::WaterToLight;
            mappings.insert(MappingType::WaterToLight, Vec::new());
            continue;
        }
        if line.contains("light-to-temperature") {
            mapping_type = MappingType::LightToTemperature;
            mappings.insert(MappingType::LightToTemperature, Vec::new());
            continue;
        }
        if line.contains("temperature-to-humidity") {
            mapping_type = MappingType::TemperatureToHumidity;
            mappings.insert(MappingType::TemperatureToHumidity, Vec::new());
            continue;
        }
        if line.contains("humidity-to-location") {
            mapping_type = MappingType::HumidityToLocation;
            mappings.insert(MappingType::HumidityToLocation, Vec::new());
            continue;
        }
        if line.is_empty() {
            mapping_type = MappingType::None;
            continue;
        }
        mappings
            .get_mut(&mapping_type)
            .expect("Mapping should exist")
            .push(Mapping::new(&line));
    }

    for seed in &mut seeds {
        seed.map_locations(&mappings);
    }
    seeds.sort_by(|a, b| b.location.cmp(&a.location));
    for seed in &seeds {
        println!("Location of seed: {}", seed.location);
    }

    Ok(())
}

fn main() {
    let _ = read_file_line_by_line("src/day5/day5_input");
}
