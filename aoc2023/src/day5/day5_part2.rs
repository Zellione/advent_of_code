use core::fmt;
use std::{
    cmp::{max, min},
    collections::BTreeMap,
    fs::File,
    io::{BufRead, BufReader},
    u64,
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

#[derive(Debug)]
struct SeedRange {
    start: u64,
    end: u64,
}

impl SeedRange {
    pub fn new(start: u64, end: u64) -> Self {
        SeedRange { start, end }
    }
}

#[derive(Debug)]
struct Mapping {
    destination: u64,
    source: u64,
    range: u64,
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
}

fn make_seeds(line: &String) -> Vec<SeedRange> {
    let mut seeds_vec: Vec<SeedRange> = Vec::new();
    let seeds: Vec<&str> = line
        .split(':')
        .last()
        .expect("should find seeds")
        .trim()
        .split(' ')
        .filter(|num| !num.trim().is_empty())
        .collect();

    let mut is_range = false;
    let mut first_num: u64 = u64::MAX;
    seeds.iter().for_each(|seed| {
        let num: u64 = seed.parse::<u64>().expect("to be a number");
        match is_range {
            false => {
                first_num = num;
                is_range = true;
            }
            true => {
                seeds_vec.push(SeedRange::new(first_num, first_num + num));
                first_num = u64::MAX;
                is_range = false;
            }
        }
    });

    seeds_vec
}

fn read_file_line_by_line(
    filepath: &str,
    mappings: &mut BTreeMap<MappingType, Vec<Mapping>>,
    seed_ranges: &mut Vec<SeedRange>,
) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open(filepath)?;
    let reader = BufReader::new(file);

    let mut mapping_type: MappingType = MappingType::None;
    for line in reader.lines() {
        let line = line.expect("should be a line");
        if line.contains("seeds:") {
            seed_ranges.extend(make_seeds(&line));
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

    Ok(())
}

fn main() {
    let mut seed_ranges: Vec<SeedRange> = Vec::new();
    let mut mappings: BTreeMap<MappingType, Vec<Mapping>> = BTreeMap::new();
    let _ = read_file_line_by_line("src/day5/day5_input", &mut mappings, &mut seed_ranges);

    for (_, mappings_of_type) in mappings {
        let mut new: Vec<SeedRange> = Vec::new();
        while seed_ranges.len() > 0 {
            let seed_range = seed_ranges.pop().expect("to have an element left");
            let mut is_mapped = false;
            for mapping in &mappings_of_type {
                let overlap_start = max(seed_range.start, mapping.source);
                let overlap_end = min(seed_range.end, mapping.source + mapping.range);
                if overlap_start < overlap_end {
                    new.push(SeedRange::new(
                        overlap_start - mapping.source + mapping.destination,
                        overlap_end - mapping.source + mapping.destination,
                    ));

                    if overlap_start > seed_range.start {
                        seed_ranges.push(SeedRange::new(seed_range.start, overlap_start));
                    }

                    if seed_range.end > overlap_end {
                        seed_ranges.push(SeedRange::new(overlap_end, seed_range.end));
                    }

                    is_mapped = true;
                }
            }

            if is_mapped == false {
                new.push(SeedRange::new(seed_range.start, seed_range.end));
            }
        }

        seed_ranges.clear();
        seed_ranges.extend(new);
    }
    seed_ranges.sort_by(|a, b| a.start.cmp(&b.start));
    println!(
        "{}",
        seed_ranges.first().expect("to has a first item").start
    );
}
