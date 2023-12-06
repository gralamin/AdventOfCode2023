extern crate filelib;

pub use filelib::load;
pub use filelib::split_lines_by_blanks;

type IdNum = u64;

pub fn example_input() -> Vec<Vec<String>> {
    return vec![
        vec!["seeds: 79 14 55 13"]
            .iter()
            .map(|s| s.to_string())
            .collect(),
        vec!["seed-to-soil map:", "50 98 2", "52 50 48"]
            .iter()
            .map(|s| s.to_string())
            .collect(),
        vec!["soil-to-fertilizer map:", "0 15 37", "37 52 2", "39 0 15"]
            .iter()
            .map(|s| s.to_string())
            .collect(),
        vec![
            "fertilizer-to-water map:",
            "49 53 8",
            "0 11 42",
            "42 0 7",
            "57 7 4",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect(),
        vec!["water-to-light map:", "88 18 7", "18 25 70"]
            .iter()
            .map(|s| s.to_string())
            .collect(),
        vec![
            "light-to-temperature map:",
            "45 77 23",
            "81 45 19",
            "68 64 13",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect(),
        vec!["temperature-to-humidity map:", "0 69 1", "1 0 69"]
            .iter()
            .map(|s| s.to_string())
            .collect(),
        vec!["humidity-to-location map:", "60 56 37", "56 93 4"]
            .iter()
            .map(|s| s.to_string())
            .collect(),
    ];
}

#[derive(PartialEq, Debug, Copy, Clone)]
enum AlmanacItem {
    Seed,
    Soil,
    Fertilizer,
    Water,
    Light,
    Temperature,
    Humidity,
    Location,
}

#[derive(PartialEq, Debug, Clone)]
struct RangeMap {
    src: AlmanacItem,
    dst: AlmanacItem,
    src_start_ids: Vec<IdNum>,
    src_end_ids: Vec<IdNum>,
    dst_start_ids: Vec<IdNum>,
    dst_end_ids: Vec<IdNum>,
}

impl RangeMap {
    pub fn new(
        src_starts: Vec<IdNum>,
        dst_starts: Vec<IdNum>,
        range_size: Vec<IdNum>,
        src_type: AlmanacItem,
        dst_type: AlmanacItem,
    ) -> RangeMap {
        let src_ends = src_starts
            .iter()
            .zip(range_size.clone())
            .map(|(src_id, cur_range)| src_id + cur_range)
            .collect();
        let dst_ends = dst_starts
            .iter()
            .zip(range_size)
            .map(|(dst_id, cur_range)| dst_id + cur_range)
            .collect();
        return RangeMap {
            src: src_type,
            dst: dst_type,
            src_start_ids: src_starts.clone(),
            src_end_ids: src_ends,
            dst_start_ids: dst_starts.clone(),
            dst_end_ids: dst_ends,
        };
    }

    fn convert_id(&self, id: IdNum) -> IdNum {
        for (index, (start_id, end_id)) in self
            .src_start_ids
            .iter()
            .zip(self.src_end_ids.clone())
            .enumerate()
        {
            if id >= *start_id && id <= end_id {
                let offset = id - *start_id;
                return self.dst_start_ids[index] + offset;
            }
        }
        // If not mapped, return same
        return id;
    }

    fn convert_range(&self, start: IdNum, end: IdNum) -> (Vec<IdNum>, Vec<IdNum>) {
        // possiblities
        //               R: |------------|
        //          |----------| [1]
        //                      |--| [2]
        //                             |------| [3]
        //          |--| [4]
        //                                   |--| [4]
        //          |-------------------------------| [5]
        let mut start_remaining = vec![start];
        let mut end_remaining = vec![end];
        let mut mapped_start = vec![];
        let mut mapped_end = vec![];

        while let Some(s_r) = start_remaining.pop() {
            let e_r = end_remaining.pop().unwrap();
            let mut map_found = false;

            for (index, (start_id, end_id)) in self
                .src_start_ids
                .iter()
                .zip(self.src_end_ids.clone())
                .enumerate()
            {
                if e_r < *start_id || s_r > end_id {
                    // We are outside the range entirely [4]
                    continue;
                }
                if s_r >= *start_id && e_r <= end_id {
                    // We are inside the range [2], just map directly
                    map_found = true;
                    let offset = s_r - *start_id;
                    let end_offset = e_r - *start_id;
                    let m_s = self.dst_start_ids[index] + offset;
                    let m_e = self.dst_start_ids[index] + end_offset;
                    mapped_start.push(m_s);
                    mapped_end.push(m_e);
                    break;
                }
                if s_r < *start_id && e_r > end_id {
                    // We are a larger range
                    map_found = true;
                    let left_end = *start_id - 1;
                    let right_start = end_id + 1;
                    let m_s = self.dst_start_ids[index];
                    let m_e = self.dst_end_ids[index];
                    start_remaining.push(s_r);
                    start_remaining.push(right_start);
                    end_remaining.push(left_end);
                    end_remaining.push(e_r);
                    mapped_start.push(m_s);
                    mapped_end.push(m_e);
                    break;
                }
                if s_r < *start_id {
                    // We extend into the range [1]
                    map_found = true;
                    let left_end = *start_id - 1;
                    let end_offset = e_r - start_id;
                    let m_s = self.dst_start_ids[index];
                    let m_e = self.dst_start_ids[index] + end_offset;
                    mapped_start.push(m_s);
                    mapped_end.push(m_e);
                    start_remaining.push(s_r);
                    end_remaining.push(left_end);
                    break;
                }
                // One possiblty left, we extend past the range [3].
                map_found = true;
                let right_start = end_id + 1;
                let offset = s_r - *start_id;
                let m_s = self.dst_start_ids[index] + offset;
                let m_e = self.dst_end_ids[index];
                mapped_start.push(m_s);
                mapped_end.push(m_e);
                start_remaining.push(right_start);
                end_remaining.push(e_r);
                break;
            }

            if !map_found {
                // These weren't found, so we just map this range to the same number
                mapped_end.push(e_r);
                mapped_start.push(s_r);
            }
        }
        return (mapped_start, mapped_end);
    }
}

// I would really like the type system to enforce this, but I can't think of a way that isn't super painful.
#[derive(PartialEq, Debug, Clone)]
struct Almanac {
    to_plant: Vec<IdNum>,
    seed_to_soil: RangeMap,
    soil_to_fertilizer: RangeMap,
    fertilizer_to_water: RangeMap,
    water_to_light: RangeMap,
    light_to_temperature: RangeMap,
    temperature_to_humidity: RangeMap,
    humidity_to_location: RangeMap,
}

fn parse_almanac(string_list: &Vec<Vec<String>>) -> Almanac {
    let mut to_plant: Vec<IdNum> = vec![];
    let mut seed_to_soil: Option<RangeMap> = None;
    let mut soil_to_fertilizer: Option<RangeMap> = None;
    let mut fertilizer_to_water: Option<RangeMap> = None;
    let mut water_to_light: Option<RangeMap> = None;
    let mut light_to_temperature: Option<RangeMap> = None;
    let mut temperature_to_humidity: Option<RangeMap> = None;
    let mut humidity_to_location: Option<RangeMap> = None;
    for grouping in string_list {
        if grouping.len() == 0 {
            // Ignore anything empty, probably a loading issue
            continue;
        }
        let first_line = grouping.first().unwrap();
        if first_line.ends_with(":") {
            // We are in a section
            let mut src_ids: Vec<IdNum> = vec![];
            let mut dst_ids: Vec<IdNum> = vec![];
            let mut ranges: Vec<IdNum> = vec![];

            // get all the numbers first.
            for line in grouping {
                if line == first_line {
                    continue;
                }
                let (dst, rest) = line.split_once(" ").unwrap();
                let (src, range) = rest.split_once(" ").unwrap();
                let dst_num: IdNum = dst.parse().unwrap();
                let src_num: IdNum = src.parse().unwrap();
                let range_num: IdNum = range.parse().unwrap();
                src_ids.push(src_num);
                dst_ids.push(dst_num);
                ranges.push(range_num);
            }

            // Now determine case
            if first_line.starts_with("seed") {
                seed_to_soil = Some(RangeMap::new(
                    src_ids,
                    dst_ids,
                    ranges,
                    AlmanacItem::Seed,
                    AlmanacItem::Soil,
                ));
            } else if first_line.starts_with("soil") {
                soil_to_fertilizer = Some(RangeMap::new(
                    src_ids,
                    dst_ids,
                    ranges,
                    AlmanacItem::Soil,
                    AlmanacItem::Fertilizer,
                ));
            } else if first_line.starts_with("fertilizer") {
                fertilizer_to_water = Some(RangeMap::new(
                    src_ids,
                    dst_ids,
                    ranges,
                    AlmanacItem::Fertilizer,
                    AlmanacItem::Water,
                ));
            } else if first_line.starts_with("water") {
                water_to_light = Some(RangeMap::new(
                    src_ids,
                    dst_ids,
                    ranges,
                    AlmanacItem::Water,
                    AlmanacItem::Light,
                ));
            } else if first_line.starts_with("light") {
                light_to_temperature = Some(RangeMap::new(
                    src_ids,
                    dst_ids,
                    ranges,
                    AlmanacItem::Light,
                    AlmanacItem::Temperature,
                ));
            } else if first_line.starts_with("temperature") {
                temperature_to_humidity = Some(RangeMap::new(
                    src_ids,
                    dst_ids,
                    ranges,
                    AlmanacItem::Temperature,
                    AlmanacItem::Humidity,
                ));
            } else if first_line.starts_with("humidity") {
                humidity_to_location = Some(RangeMap::new(
                    src_ids,
                    dst_ids,
                    ranges,
                    AlmanacItem::Humidity,
                    AlmanacItem::Location,
                ));
            }
        } else {
            // We are initial seeds
            let (_, seed_nums) = first_line.split_once("seeds: ").unwrap();
            for num in seed_nums.split(" ") {
                if num.len() == 0 {
                    continue;
                }
                let as_num: IdNum = num.parse().unwrap();
                to_plant.push(as_num);
            }
        }
    }

    return Almanac {
        to_plant: to_plant,
        seed_to_soil: seed_to_soil.unwrap(),
        soil_to_fertilizer: soil_to_fertilizer.unwrap(),
        fertilizer_to_water: fertilizer_to_water.unwrap(),
        water_to_light: water_to_light.unwrap(),
        light_to_temperature: light_to_temperature.unwrap(),
        temperature_to_humidity: temperature_to_humidity.unwrap(),
        humidity_to_location: humidity_to_location.unwrap(),
    };
}

fn almanac_to_locations(alm: &Almanac) -> Vec<IdNum> {
    let mut result = vec![];
    for seed in alm.to_plant.iter() {
        let soil = alm.seed_to_soil.convert_id(*seed);
        let fertilizer = alm.soil_to_fertilizer.convert_id(soil);
        let water = alm.fertilizer_to_water.convert_id(fertilizer);
        let light = alm.water_to_light.convert_id(water);
        let temp = alm.light_to_temperature.convert_id(light);
        let humidity = alm.temperature_to_humidity.convert_id(temp);
        let location = alm.humidity_to_location.convert_id(humidity);
        result.push(location);
    }
    return result;
}

/// Get lowest number location from this parsed nonsense.
/// ```
/// let input = day05::example_input();
/// assert_eq!(day05::puzzle_a(&input), 35);
/// ```
pub fn puzzle_a(string_list: &Vec<Vec<String>>) -> IdNum {
    let alm = parse_almanac(string_list);
    let locations = almanac_to_locations(&alm);
    return *locations.iter().min().unwrap();
}

#[derive(PartialEq, Debug, Clone)]
struct AlmanacRange {
    to_plant_start: Vec<IdNum>,
    to_plant_end: Vec<IdNum>,
    seed_to_soil: RangeMap,
    soil_to_fertilizer: RangeMap,
    fertilizer_to_water: RangeMap,
    water_to_light: RangeMap,
    light_to_temperature: RangeMap,
    temperature_to_humidity: RangeMap,
    humidity_to_location: RangeMap,
}

fn parse_almanac_range(string_list: &Vec<Vec<String>>) -> AlmanacRange {
    let mut to_plant: Vec<IdNum> = vec![];
    let mut to_plant_ends: Vec<IdNum> = vec![];
    let mut seed_to_soil: Option<RangeMap> = None;
    let mut soil_to_fertilizer: Option<RangeMap> = None;
    let mut fertilizer_to_water: Option<RangeMap> = None;
    let mut water_to_light: Option<RangeMap> = None;
    let mut light_to_temperature: Option<RangeMap> = None;
    let mut temperature_to_humidity: Option<RangeMap> = None;
    let mut humidity_to_location: Option<RangeMap> = None;
    for grouping in string_list {
        if grouping.len() == 0 {
            // Ignore anything empty, probably a loading issue
            continue;
        }
        let first_line = grouping.first().unwrap();
        if first_line.ends_with(":") {
            // We are in a section
            let mut src_ids: Vec<IdNum> = vec![];
            let mut dst_ids: Vec<IdNum> = vec![];
            let mut ranges: Vec<IdNum> = vec![];

            // get all the numbers first.
            for line in grouping {
                if line == first_line {
                    continue;
                }
                let (dst, rest) = line.split_once(" ").unwrap();
                let (src, range) = rest.split_once(" ").unwrap();
                let dst_num: IdNum = dst.parse().unwrap();
                let src_num: IdNum = src.parse().unwrap();
                let range_num: IdNum = range.parse().unwrap();
                src_ids.push(src_num);
                dst_ids.push(dst_num);
                ranges.push(range_num);
            }

            // Now determine case
            if first_line.starts_with("seed") {
                seed_to_soil = Some(RangeMap::new(
                    src_ids,
                    dst_ids,
                    ranges,
                    AlmanacItem::Seed,
                    AlmanacItem::Soil,
                ));
            } else if first_line.starts_with("soil") {
                soil_to_fertilizer = Some(RangeMap::new(
                    src_ids,
                    dst_ids,
                    ranges,
                    AlmanacItem::Soil,
                    AlmanacItem::Fertilizer,
                ));
            } else if first_line.starts_with("fertilizer") {
                fertilizer_to_water = Some(RangeMap::new(
                    src_ids,
                    dst_ids,
                    ranges,
                    AlmanacItem::Fertilizer,
                    AlmanacItem::Water,
                ));
            } else if first_line.starts_with("water") {
                water_to_light = Some(RangeMap::new(
                    src_ids,
                    dst_ids,
                    ranges,
                    AlmanacItem::Water,
                    AlmanacItem::Light,
                ));
            } else if first_line.starts_with("light") {
                light_to_temperature = Some(RangeMap::new(
                    src_ids,
                    dst_ids,
                    ranges,
                    AlmanacItem::Light,
                    AlmanacItem::Temperature,
                ));
            } else if first_line.starts_with("temperature") {
                temperature_to_humidity = Some(RangeMap::new(
                    src_ids,
                    dst_ids,
                    ranges,
                    AlmanacItem::Temperature,
                    AlmanacItem::Humidity,
                ));
            } else if first_line.starts_with("humidity") {
                humidity_to_location = Some(RangeMap::new(
                    src_ids,
                    dst_ids,
                    ranges,
                    AlmanacItem::Humidity,
                    AlmanacItem::Location,
                ));
            }
        } else {
            // We are initial seeds
            let (_, seed_nums) = first_line.split_once("seeds: ").unwrap();
            let mut range_start = 0;
            let mut range_end = 0;
            for num in seed_nums.split(" ") {
                if num.len() == 0 {
                    continue;
                }
                let as_num: IdNum = num.parse().unwrap();
                // initial condition
                if range_start == 0 && range_end == 0 {
                    range_start = as_num;
                    continue;
                }
                range_end = range_start + as_num - 1;
                to_plant.push(range_start);
                to_plant_ends.push(range_end);
                range_start = 0;
                range_end = 0;
            }
        }
    }

    return AlmanacRange {
        to_plant_start: to_plant,
        to_plant_end: to_plant_ends,
        seed_to_soil: seed_to_soil.unwrap(),
        soil_to_fertilizer: soil_to_fertilizer.unwrap(),
        fertilizer_to_water: fertilizer_to_water.unwrap(),
        water_to_light: water_to_light.unwrap(),
        light_to_temperature: light_to_temperature.unwrap(),
        temperature_to_humidity: temperature_to_humidity.unwrap(),
        humidity_to_location: humidity_to_location.unwrap(),
    };
}

/// Find the lowest number when seeds are pairs of ranges.
/// ```
/// let input = day05::example_input();
/// assert_eq!(day05::puzzle_b(&input), 46);
/// ```
pub fn puzzle_b(string_list: &Vec<Vec<String>>) -> IdNum {
    let alm = parse_almanac_range(string_list);
    let mut current_range_starts: Vec<IdNum> = alm.to_plant_start.clone();
    let mut current_range_ends: Vec<IdNum> = alm.to_plant_end.clone();
    let mut next_range_s = vec![];
    let mut next_range_e = vec![];

    while let Some(cur_start) = current_range_starts.pop() {
        let cur_end = current_range_ends.pop().unwrap();
        let (next_starts, next_ends) = alm.seed_to_soil.convert_range(cur_start, cur_end);
        assert_eq!(next_starts.len(), next_ends.len());
        next_range_s.extend(next_starts);
        next_range_e.extend(next_ends);
    }
    current_range_starts = next_range_s;
    current_range_ends = next_range_e;
    next_range_s = vec![];
    next_range_e = vec![];

    while let Some(cur_start) = current_range_starts.pop() {
        let cur_end = current_range_ends.pop().unwrap();
        let (next_starts, next_ends) = alm.soil_to_fertilizer.convert_range(cur_start, cur_end);
        assert_eq!(next_starts.len(), next_ends.len());
        next_range_s.extend(next_starts);
        next_range_e.extend(next_ends);
    }
    current_range_starts = next_range_s;
    current_range_ends = next_range_e;
    next_range_s = vec![];
    next_range_e = vec![];

    while let Some(cur_start) = current_range_starts.pop() {
        let cur_end = current_range_ends.pop().unwrap();
        let (next_starts, next_ends) = alm.fertilizer_to_water.convert_range(cur_start, cur_end);
        assert_eq!(next_starts.len(), next_ends.len());
        next_range_s.extend(next_starts);
        next_range_e.extend(next_ends);
    }
    current_range_starts = next_range_s;
    current_range_ends = next_range_e;
    next_range_s = vec![];
    next_range_e = vec![];

    while let Some(cur_start) = current_range_starts.pop() {
        let cur_end = current_range_ends.pop().unwrap();
        let (next_starts, next_ends) = alm.water_to_light.convert_range(cur_start, cur_end);
        assert_eq!(next_starts.len(), next_ends.len());
        next_range_s.extend(next_starts);
        next_range_e.extend(next_ends);
    }
    current_range_starts = next_range_s;
    current_range_ends = next_range_e;
    next_range_s = vec![];
    next_range_e = vec![];

    while let Some(cur_start) = current_range_starts.pop() {
        let cur_end = current_range_ends.pop().unwrap();
        let (next_starts, next_ends) = alm.light_to_temperature.convert_range(cur_start, cur_end);
        assert_eq!(next_starts.len(), next_ends.len());
        next_range_s.extend(next_starts);
        next_range_e.extend(next_ends);
    }
    current_range_starts = next_range_s;
    current_range_ends = next_range_e;
    next_range_s = vec![];
    next_range_e = vec![];

    while let Some(cur_start) = current_range_starts.pop() {
        let cur_end = current_range_ends.pop().unwrap();
        let (next_starts, next_ends) = alm
            .temperature_to_humidity
            .convert_range(cur_start, cur_end);
        assert_eq!(next_starts.len(), next_ends.len());
        next_range_s.extend(next_starts);
        next_range_e.extend(next_ends);
    }
    current_range_starts = next_range_s;
    current_range_ends = next_range_e;
    next_range_s = vec![];
    next_range_e = vec![];

    while let Some(cur_start) = current_range_starts.pop() {
        let cur_end = current_range_ends.pop().unwrap();
        let (next_starts, next_ends) = alm.humidity_to_location.convert_range(cur_start, cur_end);
        assert_eq!(next_starts.len(), next_ends.len());
        next_range_s.extend(next_starts);
        next_range_e.extend(next_ends);
    }
    current_range_starts = next_range_s;

    // The starts are lower then the ends, so we can just return the lowest start.
    return *current_range_starts.iter().min().unwrap();
}
