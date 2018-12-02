extern crate aoc_runner;
extern crate distance;

#[macro_use]
extern crate aoc_runner_derive;

use std::collections::{BTreeSet, HashSet};

/*#[aoc_generator(day1)]
pub fn freq_change_parser<'a>(input: &'a str) -> impl Iterator<Item=i32> + 'a {
    input.lines().map(|l| {
        l.parse::<i32>().unwrap()
    })
}

#[aoc(day1, part1)]
pub fn freq_change<T: Iterator<Item=i32>>(input: &mut T) -> i32 {
    input.by_ref().sum()
}*/

pub fn freq_change_parser<'a>(input: &'a str) -> impl Iterator<Item=i32> + Clone + 'a {
    input.lines().map(|l| {
        l.parse::<i32>().unwrap()
    })
}

#[aoc(day1, part1)]
pub fn freq_change1(input: &str) -> i32 {
    freq_change_parser(input).sum()
}

#[aoc(day1, part2, btree)]
pub fn freq_change2_btree(input: &str) -> i32 {
    let mut freq_set = BTreeSet::new();
    let mut freq = 0;

    let num_iter = freq_change_parser(input);

    for num in num_iter.cycle() {
        freq += num;

        if freq_set.contains(&freq) {
            return freq;
        } else {
            freq_set.insert(freq);
        }
    }

    panic!("No duplicates found");
}

#[aoc(day1, part2, hash)]
pub fn freq_change2_hash(input: &str) -> i32 {
    let mut freq_set = HashSet::new();
    let mut freq = 0;

    let num_iter = freq_change_parser(input);

    for num in num_iter.cycle() {
        freq += num;

        if freq_set.contains(&freq) {
            return freq;
        } else {
            freq_set.insert(freq);
        }
    }

    panic!("No duplicates found");
}

#[aoc(day2, part1)]
pub fn count_letter_reps(input: &str) -> u32 {
    let letter_freqs = &mut vec![0u8; 26];
    let mut two_rep_count = 0;
    let mut three_rep_count = 0;

    for line in input.lines() {
        for byte in line.bytes() {
            letter_freqs[(byte - 97) as usize] += 1;
        }

        let mut found_two_rep = 0;
        let mut found_three_rep = 0;

        for freq in letter_freqs.iter() {
            if *freq == 2 {
                found_two_rep = 1;
            } else if *freq == 3 {
                found_three_rep = 1;
            }
        }

        two_rep_count += found_two_rep;
        three_rep_count += found_three_rep;

        for i in 0..letter_freqs.len() {
            letter_freqs[i] = 0;
        }
    }

    two_rep_count * three_rep_count
}

#[aoc(day2, part2)]
pub fn letter_diffs(input: &str) -> String {
    let lines: Vec<&str> = input.lines().collect();

    for i in 0..lines.len() {
        for j in (i + 1)..lines.len() {
            let dist = distance::hamming(lines[i], lines[j]).unwrap();
            if dist == 1 {
                let vec = lines[i].bytes().zip(lines[j].bytes()).filter_map(|(a, b)| if a == b { Some(a) } else { None }).collect();
                return String::from_utf8(vec).unwrap();
            }
        }
    }

    panic!("No near-match found");
}


aoc_lib!{ year = 2018 }

/*#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn day1_part1() {
        let input = "+1\n-2";
        let mut parsed = freq_change_parser(input);
        let result = freq_change(&mut parsed);
        assert_eq!(result, -1)
    }
}
*/
