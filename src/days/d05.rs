use std::str::Lines;
use std::{
    sync::{Arc, Mutex},
    thread,
};

use super::{Answer, Day, DayImpl};

const CURRENT_DAY: u8 = 5;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Range {
    source_start: u32,
    destination_start: u32,
    length: u32,
}

impl Range {
    fn contains_source(&self, value: u32) -> bool {
        (self.source_start..(self.source_start + self.length)).contains(&value)
    }

    fn translate(&self, value: u32) -> u32 {
        value - self.source_start + self.destination_start
    }
}

#[derive(Debug, Clone)]
pub struct Map(Vec<Range> /*, HashMap<u32, u32>*/);

impl Map {
    fn translate_value(&self, value: u32) -> u32 {
        /*if self.1.contains_key(&value) {
            println!("Cache hit!");     // => Never actually reported a Cache hit - instead my 32 GB of memory get completely filled. So Caching is irrelevant here.
            return *self.1.get(&value).unwrap();
        }*/

        for range in &self.0 {
            if range.contains_source(value) {
                let translated = range.translate(value);
                //self.1.insert(value, translated);
                return translated;
            }
        }

        //self.1.insert(value, value);
        value
    }

    fn parse_ranges(lines: &mut Lines) -> Self {
        let mut out = Vec::with_capacity(lines.clone().count());

        for line in lines.by_ref() {
            if line.is_empty() {
                break;
            }

            let mut splitter = line.split_whitespace();

            out.push(Range {
                destination_start: splitter.next().unwrap().parse().unwrap(),
                source_start: splitter.next().unwrap().parse().unwrap(),
                length: splitter.next().unwrap().parse().unwrap(),
            })
        }

        out.sort();

        Self(out /*, HashMap::new()*/)
    }
}

#[derive(Debug, Clone)]
pub struct Almanac {
    seeds: Vec<u32>,
    seed_to_soil: Map,
    soil_to_fertilizer: Map,
    fertilizer_to_water: Map,
    water_to_light: Map,
    light_to_temperature: Map,
    temperature_to_humidity: Map,
    humidity_to_location: Map,
}

impl Almanac {
    fn seed_to_destination(&self, seed: u32) -> u32 {
        self.humidity_to_location.translate_value(
            self.temperature_to_humidity.translate_value(
                self.light_to_temperature.translate_value(
                    self.water_to_light.translate_value(
                        self.fertilizer_to_water.translate_value(
                            self.soil_to_fertilizer
                                .translate_value(self.seed_to_soil.translate_value(seed)),
                        ),
                    ),
                ),
            ),
        )
    }
}

impl From<&str> for Almanac {
    fn from(value: &str) -> Self {
        let mut lines = value.lines();
        let seeds = lines
            .next()
            .unwrap()
            .split_once(": ")
            .unwrap()
            .1
            .split(' ')
            .map(|v| v.parse().unwrap())
            .collect();

        let mut seed_to_soil = None;
        let mut soil_to_fertilizer = None;
        let mut fertilizer_to_water = None;
        let mut water_to_light = None;
        let mut light_to_temperature = None;
        let mut temperature_to_humidity = None;
        let mut humidity_to_location = None;

        while let Some(line) = lines.next() {
            match line {
                "seed-to-soil map:" => seed_to_soil = Some(Map::parse_ranges(&mut lines)),
                "soil-to-fertilizer map:" => {
                    soil_to_fertilizer = Some(Map::parse_ranges(&mut lines))
                }
                "fertilizer-to-water map:" => {
                    fertilizer_to_water = Some(Map::parse_ranges(&mut lines))
                }
                "water-to-light map:" => water_to_light = Some(Map::parse_ranges(&mut lines)),
                "light-to-temperature map:" => {
                    light_to_temperature = Some(Map::parse_ranges(&mut lines))
                }
                "temperature-to-humidity map:" => {
                    temperature_to_humidity = Some(Map::parse_ranges(&mut lines))
                }
                "humidity-to-location map:" => {
                    humidity_to_location = Some(Map::parse_ranges(&mut lines))
                }
                "" => continue,
                _ => panic!("unexpected input"),
            }
        }

        Almanac {
            seeds,
            seed_to_soil: seed_to_soil.unwrap(),
            soil_to_fertilizer: soil_to_fertilizer.unwrap(),
            fertilizer_to_water: fertilizer_to_water.unwrap(),
            water_to_light: water_to_light.unwrap(),
            light_to_temperature: light_to_temperature.unwrap(),
            temperature_to_humidity: temperature_to_humidity.unwrap(),
            humidity_to_location: humidity_to_location.unwrap(),
        }
    }
}

type Data = Almanac;
impl DayImpl<Data> for Day<CURRENT_DAY> {
    fn init_test() -> (Self, Data) {
        Self::init(include_str!("test_inputs/test05.txt"))
    }

    fn expected_results() -> (Answer, Answer) {
        (Answer::Number(35), Answer::Number(46))
    }

    fn init(input: &str) -> (Self, Data) {
        (Self {}, Almanac::from(input))
    }

    fn one(&self, data: &mut Data) -> Answer {
        let mut lowest = u32::MAX;

        for seed in data.seeds.clone() {
            lowest = lowest.min(data.seed_to_destination(seed));
        }

        Answer::Number(lowest as u64)
    }

    fn two(&self, data: &mut Data) -> Answer {
        let lowest = Arc::new(Mutex::new(u32::MAX));

        let seeds = data.seeds.clone();
        let mut iter = seeds.iter();

        let mut thread_handles = Vec::with_capacity(seeds.len() / 2);

        while let Some(range_start) = iter.next() {
            let local_range_start = *range_start;
            let local_len = *iter.next().unwrap();
            let local_data = data.clone();

            let shared_lowest = Arc::clone(&lowest);

            thread_handles.push(thread::spawn(move || {
                let mut local_lowest = u32::MAX;

                for seed in local_range_start..(local_range_start + local_len) {
                    let location = local_data.seed_to_destination(seed);
                    local_lowest = local_lowest.min(location);
                }

                let mut lowest = shared_lowest.lock().unwrap();
                *lowest = lowest.min(local_lowest);
            }))
        }

        for handle in thread_handles {
            handle.join().unwrap();
        }

        let result = *lowest.lock().unwrap();
        Answer::Number(result as u64)
    }
}
