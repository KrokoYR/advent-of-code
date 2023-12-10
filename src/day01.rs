use aoc_runner_derive::{aoc, aoc_generator};
use once_cell::sync::Lazy;
use std::collections::{HashMap, VecDeque};

use crate::trie::Trie;

#[aoc_generator(day1, part1)]
pub fn input_generator(input: &str) -> Vec<u32> {
    input
        .lines()
        .map(|line| {
            let first = line.chars().find(|ch| ch.is_ascii_digit()).unwrap();
            let last = line.chars().rfind(|ch| ch.is_ascii_digit()).unwrap();

            first.to_digit(10).unwrap() as u32 * 10 + last.to_digit(10).unwrap() as u32
        })
        .collect()
}

#[aoc(day1, part1)]
pub fn part1(input: &[u32]) -> u32 {
    input.iter().sum()
}

static STR_DIGITS: Lazy<HashMap<&str, u32>> = Lazy::new(|| {
    let mut map = HashMap::new();
    map.insert("one", 1);
    map.insert("two", 2);
    map.insert("three", 3);
    map.insert("four", 4);
    map.insert("five", 5);
    map.insert("six", 6);
    map.insert("seven", 7);
    map.insert("eight", 8);
    map.insert("nine", 9);

    map
});

static FORWARD_TRIE: Lazy<Trie> = Lazy::new(|| {
    let mut trie = Trie::new();
    for (_, word) in STR_DIGITS.iter().enumerate() {
        trie.insert(word.0);
    }

    trie
});
static BACKWARD_TRIE: Lazy<Trie> = Lazy::new(|| {
    let mut trie = Trie::new();
    for (_, word) in STR_DIGITS.iter().enumerate() {
        trie.insert(word.0.chars().rev().collect::<String>().as_str());
    }

    trie
});

#[aoc_generator(day1, part2)]
pub fn input_generator_part2(input: &str) -> Vec<u32> {
    fn find(input: &str, trie: &Trie) -> u32 {
        let mut maybe_digit = VecDeque::with_capacity(5);
        for ch in input.chars() {
            if ch.is_ascii_digit() {
                return ch.to_digit(10).unwrap();
            }

            maybe_digit.push_back(ch);
            if maybe_digit.as_slices().1.len() > 0 {
                maybe_digit.make_contiguous();
            }

            let (has, last) = trie.contains_chars(&maybe_digit.as_slices().0);

            if has && last {
                return *STR_DIGITS
                    .get(&maybe_digit.iter().collect::<String>().as_str())
                    .unwrap();
            } else if has {
                continue;
            } else {
                while let Some(_) = maybe_digit.pop_front() {
                    let (has, _) = trie.contains_chars(&maybe_digit.as_slices().0);
                    if has {
                        break;
                    }
                }
            }
        }
        0
    }

    fn find_reverse(input: &str, trie: &Trie) -> u32 {
        let mut maybe_digit = VecDeque::with_capacity(5);
        for ch in input.chars().rev() {
            if ch.is_ascii_digit() {
                return ch.to_digit(10).unwrap();
            }

            maybe_digit.push_back(ch);
            if maybe_digit.as_slices().1.len() > 0 {
                maybe_digit.make_contiguous();
            }

            let (has, last) = trie.contains_chars(&maybe_digit.as_slices().0);
            if has && last {
                match STR_DIGITS.get(&maybe_digit.iter().rev().collect::<String>().as_str()) {
                    Some(digit) => {
                        return *digit;
                    }
                    None => {
                        panic!("Not found");
                    }
                }
            } else if has {
                continue;
            } else {
                while let Some(_) = maybe_digit.pop_front() {
                    let (has, _) = trie.contains_chars(&maybe_digit.as_slices().0);
                    if has {
                        break;
                    }
                }
            }
        }
        0
    }

    input
        .lines()
        .map(|line| {
            let first = find(line, &FORWARD_TRIE);
            let last = find_reverse(line, &BACKWARD_TRIE);

            first * 10 + last
        })
        .collect()
}

#[aoc(day1, part2)]
pub fn part2(input: &[u32]) -> u32 {
    input.iter().sum()
}
