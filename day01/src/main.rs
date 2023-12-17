use std::ops::Not;

use utils::parse_text;

fn main() {
    let text = parse_text();
    let lines = get_lines(&text);
    let calibration_numbers = get_calibration_numbers(&lines);
    let transformed_lines = transform_numbers(&lines);
    let mod_calibration_numbers = get_calibration_numbers(&transformed_lines);

    if let Some(cal_numbers) = calibration_numbers {
        let calibration_sum: u32 = cal_numbers.into_iter().sum();
        println!(
            "The sum of original calibration values is {}",
            calibration_sum
        );
    } else {
        println!("The sum of original calibration values is undefined");
    }

    if let Some(mod_cal_numbers) = mod_calibration_numbers {
        let calibration_sum: u32 = mod_cal_numbers.into_iter().sum();
        println!(
            "The sum of transformed calibration values is {}",
            calibration_sum
        );
    } else {
        println!("The sum of transformed calibration values is undefined");
    }
}

fn get_lines(text: &str) -> Vec<String> {
    text.replace(' ', "")
        .split('\n')
        .map(String::from)
        .collect()
}

fn transform_numbers(lines: &[String]) -> Vec<String> {
    lines.iter().map(extract_digits).collect()
}

fn extract_digits(line: &String) -> String {
    let mut res = Vec::new();
    let chars = line.chars().collect::<Vec<_>>();
    for i in 0..line.len() {
        if chars[i].is_ascii_digit() {
            res.push(chars[i].to_digit(10).unwrap())
        }
        let rest: String = chars[i..].iter().collect();
        if rest.starts_with("one") {
            res.push(1);
        } else if rest.starts_with("two") {
            res.push(2);
        } else if rest.starts_with("three") {
            res.push(3);
        } else if rest.starts_with("four") {
            res.push(4);
        } else if rest.starts_with("five") {
            res.push(5);
        } else if rest.starts_with("six") {
            res.push(6);
        } else if rest.starts_with("seven") {
            res.push(7);
        } else if rest.starts_with("eight") {
            res.push(8);
        } else if rest.starts_with("nine") {
            res.push(9);
        }
    }
    res.into_iter()
        .filter_map(|i| char::from_digit(i, 10))
        .collect()
}

fn get_calibration_numbers(lines: &[String]) -> Option<Vec<u32>> {
    lines
        .iter()
        .map(|line| get_calibration_number(line))
        .collect()
}

fn get_calibration_number(line: &str) -> Option<u32> {
    let filtered = line
        .chars()
        .filter_map(|c| c.to_digit(10))
        .collect::<Vec<u32>>();

    filtered.is_empty().not().then(|| {
        10 * filtered.first().expect("First digit must exist")
            + filtered.last().expect("Last digit must exist")
    })
}
