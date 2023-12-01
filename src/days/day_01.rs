use std::str::FromStr;

use indexmap::IndexMap;

use super::Day;

pub struct Day01;

impl Day for Day01 {
    fn file_name(&self) -> String {
        String::from("day_01.txt")
    }

    fn solution_1(&self, lines: &Vec<String>) -> usize {
        let total_calibration_values: usize = lines
            .iter()
            .map(|line| {
                let mut line_chars = line
                    .chars()
                    .filter(|c| c.is_digit(10))
                    .collect::<Vec<char>>();
                if line_chars.len() < 2 {
                    line_chars.push(line_chars[0]);
                }
                let line_chars = vec![line_chars.first().unwrap(), line_chars.last().unwrap()];
                return usize::from_str(&String::from_iter(line_chars)).unwrap();
            })
            .sum();
        total_calibration_values
    }

    fn solution_2(&self, lines: &Vec<String>) -> usize {
        let mut spelled_digit_map = IndexMap::new();
        spelled_digit_map.insert("one", 1);
        spelled_digit_map.insert("two", 2);
        spelled_digit_map.insert("three", 3);
        spelled_digit_map.insert("four", 4);
        spelled_digit_map.insert("five", 5);
        spelled_digit_map.insert("six", 6);
        spelled_digit_map.insert("seven", 7);
        spelled_digit_map.insert("eight", 8);
        spelled_digit_map.insert("nine", 9);

        let mut digit_map = IndexMap::new();
        digit_map.insert("1", 1);
        digit_map.insert("2", 2);
        digit_map.insert("3", 3);
        digit_map.insert("4", 4);
        digit_map.insert("5", 5);
        digit_map.insert("6", 6);
        digit_map.insert("7", 7);
        digit_map.insert("8", 8);
        digit_map.insert("9", 9);

        lines.iter().fold(0, |acc, val| {
            acc + find_number(val, &digit_map, &spelled_digit_map)
        })
    }
}

fn find_number(
    line: &String,
    digit_map: &IndexMap<&str, usize>,
    spelled_digit_map: &IndexMap<&str, usize>,
) -> usize {
    // (index of first match, match)
    let mut occurrences: Vec<(usize, &str)> = vec![];

    occurrences.append(&mut get_occurrences_from_line(line, spelled_digit_map));
    occurrences.append(&mut get_occurrences_from_line(line, digit_map));
    occurrences.sort_by(|val_a, val_b| val_a.0.cmp(&val_b.0));

    let first_occurrence = occurrences.first().unwrap();
    let last_occurrence = occurrences.last().unwrap();

    let (val_1, val_2) = convert_occurrences_to_values(
        first_occurrence,
        last_occurrence,
        digit_map,
        spelled_digit_map,
    );

    let total = val_1 * 10 + val_2;

    return total;
}

fn convert_occurrences_to_values(
    first_occurrence: &(usize, &str),
    last_occurrence: &(usize, &str),
    digit_map: &IndexMap<&str, usize>,
    spelled_digit_map: &IndexMap<&str, usize>,
) -> (usize, usize) {
    let val_1;
    let val_2;

    if digit_map.contains_key(first_occurrence.1) {
        val_1 = *digit_map.get(first_occurrence.1).unwrap();
    } else {
        val_1 = *spelled_digit_map.get(first_occurrence.1).unwrap();
    }
    if digit_map.contains_key(last_occurrence.1) {
        val_2 = *digit_map.get(last_occurrence.1).unwrap();
    } else {
        val_2 = *spelled_digit_map.get(last_occurrence.1).unwrap();
    }

    (val_1, val_2)
}

fn get_occurrences_from_line<'a>(
    line: &'a String,
    reference_map: &IndexMap<&str, usize>,
) -> Vec<(usize, &'a str)> {
    let mut all_occurrences: Vec<(usize, &str)> = vec![];

    for value in reference_map.keys() {
        let mut occurrences = line.match_indices(value).collect::<Vec<(usize, &str)>>();

        if !occurrences.is_empty() {
            all_occurrences.append(&mut occurrences);
        }
    }

    return all_occurrences;
}
