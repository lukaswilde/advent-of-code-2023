use std::str::FromStr;

use anyhow::Result;

use utils::parse_text;

#[derive(Debug)]
struct RangeMap {
    src_start: usize,
    dst_start: usize,
    size: usize,
}

impl RangeMap {
    fn lookup(&self, src: usize) -> Option<usize> {
        if src < self.src_start {
            return None;
        }
        if src >= self.src_start + self.size {
            return None;
        }
        let offset = src - self.src_start;
        Some(self.dst_start + offset)
    }

    fn rlookup(&self, dst: usize) -> Option<usize> {
        if dst < self.dst_start {
            return None;
        }
        if dst >= self.dst_start + self.size {
            return None;
        }
        let offset = dst - self.dst_start;
        Some(self.src_start + offset)
    }
}

#[derive(Debug)]
struct CategoryMap {
    ranges: Vec<RangeMap>,
}

impl CategoryMap {
    fn lookup(&self, src: usize) -> usize {
        for map in &self.ranges {
            if let Some(dst) = map.lookup(src) {
                return dst;
            }
        }
        src
    }

    fn rlookup(&self, dst: usize) -> usize {
        for map in &self.ranges {
            if let Some(src) = map.rlookup(dst) {
                return src;
            }
        }
        dst
    }
}

impl FromStr for CategoryMap {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let ranges = s
            .lines()
            .skip(1) // skip name of map (e.g. "seed-to-soil map")
            .map(|line| {
                let mut numbers = line.split_whitespace().map(|str| {
                    str.parse::<usize>()
                        .expect("Should be convertible to usize")
                });
                RangeMap {
                    dst_start: numbers.next().expect("There should be a dst_start"),
                    src_start: numbers.next().expect("There should be a src_start"),
                    size: numbers.next().expect("There should be a range length"),
                }
            })
            .collect();
        Ok(CategoryMap { ranges })
    }
}

fn get_chunks(text: &str) -> Vec<&str> {
    text.split("\n\n").collect()
}

fn create_maps(chunks: &[&str]) -> Vec<CategoryMap> {
    chunks
        .iter()
        .map(|chunk| {
            chunk
                .parse::<CategoryMap>()
                .expect("Chunk should be convertible to CategoryMap")
        })
        .collect()
}

fn get_seeds(line: &str) -> Vec<usize> {
    line.split_whitespace()
        .skip(1)
        .map(|str| {
            str.parse::<usize>()
                .expect("Seeds should be convertible to u32")
        })
        .collect()
}

fn get_seed_ranges(line: &str) -> Vec<(usize, usize)> {
    line.split_whitespace()
        .skip(1)
        .collect::<Vec<_>>()
        .chunks(2)
        .map(|chunk| {
            let start = chunk[0].parse().expect("seed start should be convertible");
            let size: usize = chunk[1].parse().expect("seed range should be convertible");
            (start, start + size)
        })
        .collect()
}

fn get_location_ranges(map: &CategoryMap) -> Vec<(usize, usize)> {
    map.ranges
        .iter()
        .map(|range| (range.dst_start, range.dst_start + range.size))
        .collect()
}

fn get_min_location(seeds: &[usize], maps: &[CategoryMap]) -> usize {
    seeds
        .iter()
        .map(|&seed| {
            let mut current = seed;
            for map in maps {
                current = map.lookup(current);
            }
            current
        })
        .min()
        .expect("Location iterator should not be empty")
}

fn is_in_seeds(seeds: &[(usize, usize)], value: usize) -> bool {
    seeds.iter().any(|r| r.0 <= value && value < r.1)
}

fn get_min_location_by_ranges(
    seeds: &[(usize, usize)],
    maps: &[CategoryMap],
    max_location: usize,
) -> usize {
    let found = (0..max_location).find(|&location| {
        let mut current = location;
        for map in maps.iter().rev() {
            current = map.rlookup(current);
        }
        is_in_seeds(seeds, current)
    });
    match found {
        Some(seed) => seed,
        None => unreachable!(),
    }
}

fn main() {
    let text = parse_text();
    let chunks = get_chunks(&text);
    assert_eq!(chunks.len(), 8);

    let seeds = get_seeds(chunks[0]);
    let maps = create_maps(&chunks[1..]);
    let min_location = get_min_location(&seeds, &maps);

    println!("The closest location is {}", min_location);

    let seeds_ranges = get_seed_ranges(chunks[0]);
    let location_ranges =
        get_location_ranges(maps.last().expect("Last map should be for locations"));
    let max_location = location_ranges
        .into_iter()
        .map(|r| r.1)
        .max()
        .expect("Location range iterator should not be empty");
    let range_min_location = get_min_location_by_ranges(&seeds_ranges, &maps, max_location);

    println!(
        "The closest location using ranges is {}",
        range_min_location
    );
}
