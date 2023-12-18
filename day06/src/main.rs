use utils::parse_text;

#[derive(Debug)]
struct Race {
    time: usize,
    distance: usize,
}

fn get_races(text: &str) -> Vec<Race> {
    let (times, distances) = text.split_once('\n').expect("Input should have two lines");
    let times = times.split_whitespace().skip(1).map(|num| {
        num.parse::<usize>()
            .expect("Times should be convertible to usize")
    });
    let distances = distances.split_whitespace().skip(1).map(|num| {
        num.parse::<usize>()
            .expect("Distances should be convertible to usize")
    });
    times
        .zip(distances)
        .map(|(time, distance)| Race { time, distance })
        .collect()
}

fn get_single_race(text: &str) -> Race {
    let (time, distance) = text.split_once('\n').expect("Input should have two lines");
    let time = extract_number(time);
    let distance = extract_number(distance);

    Race { time, distance }
}

fn extract_number(line: &str) -> usize {
    line.split_whitespace()
        .skip(1)
        .fold(String::new(), |acc, s| format!("{}{}", acc, s))
        .parse::<usize>()
        .expect("Time should be convertible to number")
}

fn find_num_valid_velocities(race: &Race) -> usize {
    let lb = (0..=race.time)
        .find(|time| (race.time - time) * time > race.distance)
        .expect("There should be a valid time to hold down the button");
    let ub = (0..=race.time)
        .rfind(|time| (race.time - time) * time > race.distance)
        .expect("There should be a valid time to hold down the button");
    ub - lb + 1
}

fn get_margin_of_error(races: &[Race]) -> usize {
    races
        .iter()
        .map(|race| find_num_valid_velocities(race))
        .product()
}

fn main() {
    let text = parse_text();
    let races = get_races(&text);
    let single_race = get_single_race(&text);
    let margin_of_error = get_margin_of_error(&races);
    let single_margin_of_error = get_margin_of_error(&[single_race]);
    println!("The margin of error is {}", margin_of_error);
    println!(
        "The margin of error with correct kerning is {}",
        single_margin_of_error
    );
}
