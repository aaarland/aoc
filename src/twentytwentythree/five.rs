use crate::solutions::{Part, Solution, UpdateFn};
pub struct DayFive;

impl Solution for DayFive {
    fn solve(&self, lines: Vec<String>, part: Part, _: Option<UpdateFn>) -> String {
        match part {
            Part::One => part_one(lines).to_string(),
            Part::Two => part_two(lines).to_string(),
        }
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
                    return almanac.destination - almanac.source + m;
                }
            }
            *m
        })
        .collect::<Vec<_>>()
}

fn find_in_map_reverse(map: Vec<i64>, almanacs: &Vec<Almanac>) -> Vec<i64> {
    map.iter()
        .map(|m| {
            for almanac in almanacs {
                if m >= &almanac.destination && m < &(almanac.destination + almanac.range) {
                    return almanac.source - almanac.destination + m;
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
    let temperature_to_humidity_map =
        find_in_map(light_to_temperature_map, temperature_to_humidity);
    let humidity_to_location_map = find_in_map(temperature_to_humidity_map, humidity_to_location);
    humidity_to_location_map.iter().min().unwrap().clone()
}

fn part_two(lines: Vec<String>) -> i64 {
    let seed_ranges = get_seeds(lines[0].clone());
    let seed_to_soil: Vec<Almanac> = create_almanacs(&lines, "seed-to-soil");
    let soil_to_fertilizer: Vec<Almanac> = create_almanacs(&lines, "soil-to-fertilizer");
    let fertilizer_to_water: Vec<Almanac> = create_almanacs(&lines, "fertilizer-to-water");
    let water_to_light = create_almanacs(&lines, "water-to-light");
    let light_to_temperature = create_almanacs(&lines, "light-to-temperature");
    let temperature_to_humidity = create_almanacs(&lines, "temperature-to-humidity");
    let humidity_to_location = create_almanacs(&lines, "humidity-to-location");
    println!("creating locations");
    let mut locations = humidity_to_location
        .iter()
        .flat_map(|l| (l.destination..l.destination + l.range).collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let min = locations.iter().min().unwrap();
    locations.append(&mut (1..*min).collect::<Vec<_>>());

    println!("creating maps");
    let location_to_humidity_map = find_in_map_reverse(locations, &humidity_to_location);
    let humidity_to_temperature_map =
        find_in_map_reverse(location_to_humidity_map, &temperature_to_humidity);
    let temperature_to_light_map =
        find_in_map_reverse(humidity_to_temperature_map, &light_to_temperature);
    let light_to_water_map = find_in_map_reverse(temperature_to_light_map, &water_to_light);
    let water_to_fertilizer_map = find_in_map_reverse(light_to_water_map, &fertilizer_to_water);
    let fertilizer_to_soil_map = find_in_map_reverse(water_to_fertilizer_map, &soil_to_fertilizer);
    let soil_to_seed_map = find_in_map_reverse(fertilizer_to_soil_map, &seed_to_soil);

    let mut seeds = Vec::new();
    let mut i = 0;

    println!("creating seeds");
    while i < seed_ranges.len() - 1 {
        println!("{:?}", i);
        let start = seed_ranges[i];
        let end = seed_ranges[i] + seed_ranges[i + 1];
        for seed in soil_to_seed_map.iter() {
            if seed >= &start && seed < &end {
                seeds.push(*seed);
            }
        }
        i += 2;
    }

    let seed_to_soil_map = find_in_map(seeds, seed_to_soil);
    let soil_to_fertilizer_map = find_in_map(seed_to_soil_map, soil_to_fertilizer);
    let fertilizer_to_water_map = find_in_map(soil_to_fertilizer_map, fertilizer_to_water);
    let water_to_light_map = find_in_map(fertilizer_to_water_map, water_to_light);
    let light_to_temperature_map = find_in_map(water_to_light_map, light_to_temperature);
    let temperature_to_humidity_map =
        find_in_map(light_to_temperature_map, temperature_to_humidity);
    let humidity_to_location_map = find_in_map(temperature_to_humidity_map, humidity_to_location);
    humidity_to_location_map.iter().min().unwrap().clone()
}

#[cfg(test)]
mod tests {
    use crate::utils::read_file;

    use super::*;

    #[tokio::test]
    async fn test_part_one() {
        let lines = read_file(&"2023/example5".to_string()).await;
        assert_eq!(part_one(lines), 35);
    }

    #[tokio::test]
    async fn test_part_two() {
        let lines = read_file(&"2023/example5".to_string()).await;
        assert_eq!(part_two(lines), 46);
    }
}
