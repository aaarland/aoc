use crate::solutions::Solution;
pub struct DayFive;

impl Solution for DayFive {
    fn solve(&self, lines: Vec<String>) -> () {
        let start_time = std::time::Instant::now();
        let part_one_answer = part_one(lines.clone());
        let elapsed = start_time.elapsed();
        println!(
            "Day 5 part 1 answer: {} ({:?})",
            part_one_answer, elapsed
        );
        let start_time = std::time::Instant::now();
        let part_two_answer = part_two(lines);
        let elapsed = start_time.elapsed();
        println!(
            "Day 5 part 2 answer: {} ({:?})",
            part_two_answer, elapsed
        );
    }
}

#[derive(Debug)]
struct Almanac {
    destination: i64,
    source: i64,
    range: i64,
}

fn get_seeds(line: String) -> Vec<i64> {
    let mut seeds: Vec<i64> = Vec::new();
    for seed in line.split(":").nth(1).unwrap().split_whitespace() {
        seeds.push(seed.parse::<i64>().unwrap());
    }
    seeds
}

fn create_almanacs(lines: &Vec<String>, map: &str) -> Vec<Almanac> {
    lines
        .iter()
        .skip_while(|line| !line.contains(map))
        .skip(1)
        .take_while(|line| "" != line.as_str())
        .map(|line| {
            let mut almanac = line.split_whitespace();
            Almanac {
                destination: almanac.next().unwrap().parse::<i64>().unwrap(),
                source: almanac.next().unwrap().parse::<i64>().unwrap(),
                range: almanac.next().unwrap().parse::<i64>().unwrap(),
            }
        })
        .collect()
}

fn find_in_map(map: Vec<i64>, almanacs: Vec<Almanac>) -> Vec<i64> {
    map.iter()
        .map(|m| {
            for almanac in &almanacs {
                if m >= &almanac.source && m < &(almanac.source + almanac.range) {
                    println!("{} {} {}", m, almanac.destination, almanac.source);
                    println!("{}", almanac.destination - almanac.source + m);
                    return almanac.destination - almanac.source + m;
                }
            }
            *m
        })
        .collect::<Vec<_>>()
}

fn part_one(lines: Vec<String>) -> i64 {
    let seeds = get_seeds(lines[0].clone());
    let seed_to_soil: Vec<Almanac> = create_almanacs(&lines, "seed-to-soil");
    let soil_to_fertilizer: Vec<Almanac> = create_almanacs(&lines, "soil-to-fertilizer");
    let fertilizer_to_water: Vec<Almanac> = create_almanacs(&lines, "fertilizer-to-water");
    let water_to_light = create_almanacs(&lines, "water-to-light");
    let light_to_temperature = create_almanacs(&lines, "light-to-temperature");
    let temperature_to_humidity = create_almanacs(&lines, "temperature-to-humidity");
    let humidity_to_location = create_almanacs(&lines, "humidity-to-location");

    let seed_to_soil_map = find_in_map(seeds, seed_to_soil);
    let soil_to_fertilizer_map = find_in_map(seed_to_soil_map, soil_to_fertilizer);
    let fertilizer_to_water_map = find_in_map(soil_to_fertilizer_map, fertilizer_to_water);
    let water_to_light_map = find_in_map(fertilizer_to_water_map, water_to_light);
    let light_to_temperature_map = find_in_map(water_to_light_map, light_to_temperature);
    let temperature_to_humidity_map = find_in_map(light_to_temperature_map, temperature_to_humidity);
    let humidity_to_location_map = find_in_map(temperature_to_humidity_map, humidity_to_location);
    humidity_to_location_map.iter().min().unwrap().clone()
}

fn part_two(lines: Vec<String>) -> i64 {
    0
}

#[cfg(test)]
mod tests {
    use crate::utils::read_file;

    use super::*;

    #[test]
    fn test_part_one() {
        let lines = read_file(&"2023/example5".to_string());
        assert_eq!(part_one(lines), 35);
    }

    #[test]
    fn test_part_two() {}
}
