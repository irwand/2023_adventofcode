use std::fs::File;
use std::io::{BufRead, BufReader};

fn _open_file_and_return_lines(filepath: String) -> Vec<String> {
    let file = File::open(filepath).unwrap();
    let reader = BufReader::new(file);
    let mut lines = Vec::new();

    for line in reader.lines() {
        lines.push(line.unwrap());
    }

    lines // Return the lines vector
}

struct Map {
    dest: u64,
    source: u64,
    len: u64,
}

impl Map {
    fn new(dest: u64, source: u64, len: u64) -> Map {
        Map {
            dest: dest,
            source: source,
            len: len,
        }
    }

    fn get_dest(&self, source: u64) -> Result<u64, i32> {
        if source >= self.source && source < self.source + self.len {
            return Ok(self.dest + (source - self.source));
        }
        Err(-1)
    }
}

fn get_dest(map: &Vec<Map>, source: u64) -> u64 {
    for m in map.iter() {
        if let Ok(dest) = m.get_dest(source) {
            return dest;
        }
    }
    source
}

fn main() {
    // pass first argument to function
    let lines = _open_file_and_return_lines(std::env::args().nth(1).unwrap());
    let mut seeds = Vec::new();
    let mut seed_to_soil: Vec<Map> = Vec::new();
    let mut soil_to_fertilizer: Vec<Map> = Vec::new();
    let mut fertilizer_to_water: Vec<Map> = Vec::new();
    let mut water_to_light: Vec<Map> = Vec::new();
    let mut light_to_temperature: Vec<Map> = Vec::new();
    let mut temperature_to_humidity: Vec<Map> = Vec::new();
    let mut humidity_to_location: Vec<Map> = Vec::new();
    let mut fillvec: &mut Vec<Map> = &mut seed_to_soil;
    for line in lines.iter() {
        if line.starts_with("seeds:") {
            for s in line[6..].to_string().trim().split(" ") {
                seeds.push(s.parse::<u64>().unwrap());
            }
        } else if line.starts_with("seed-to-soil map:") {
            fillvec = &mut seed_to_soil;
        } else if line.starts_with("soil-to-fertilizer map:") {
            fillvec = &mut soil_to_fertilizer;
        } else if line.starts_with("fertilizer-to-water map:") {
            fillvec = &mut fertilizer_to_water;
        } else if line.starts_with("water-to-light map:") {
            fillvec = &mut water_to_light;
        } else if line.starts_with("light-to-temperature map:") {
            fillvec = &mut light_to_temperature;
        } else if line.starts_with("temperature-to-humidity map:") {
            fillvec = &mut temperature_to_humidity;
        } else if line.starts_with("humidity-to-location map:") {
            fillvec = &mut humidity_to_location;
        } else if line.is_empty() {
            continue;
        } else {
            let mut split = line.split(" ");
            fillvec.push(Map::new(
                split.next().unwrap().parse::<u64>().unwrap(),
                split.next().unwrap().parse::<u64>().unwrap(),
                split.next().unwrap().parse::<u64>().unwrap(),
            ));
        }
    }

    let mut min_location: u64 = 9999999999999999;
    let mut start: u64 = 0;
    seeds.into_iter().for_each(|seed| {
        if start == 0 {
            start = seed;
            println!("seed: {}", seed);
        } else {
            println!("looping: {}", seed);
            for s in start..(start + seed) {
                let soil = get_dest(&seed_to_soil, s);
                let fertilizer = get_dest(&soil_to_fertilizer, soil);
                let water = get_dest(&fertilizer_to_water, fertilizer);
                let light = get_dest(&water_to_light, water);
                let temperature = get_dest(&light_to_temperature, light);
                let humidity = get_dest(&temperature_to_humidity, temperature);
                let location = get_dest(&humidity_to_location, humidity);
                if location < min_location {
                    min_location = location;
                }
            }
            start = 0;
            println!("min location: {}", min_location)
        }
    });

    println!("done, min location: {}", min_location)
}
