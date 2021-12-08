use anyhow::{anyhow, Error, Result};
use std::{collections::HashSet, str::FromStr};

fn main() {
    const INPUT: &str = include_str!("input.txt");
    // The input is a newline-separated list of pipe-separated signal | output combinations
    let entries: Vec<Entry> = INPUT
        .lines()
        .map(|line| {
            let (signal, output) = line.split_once(" | ").unwrap();
            Entry::new(signal, output)
        })
        .collect();

    // Part 1: count the number of digits 1, 4, 7, 8 in the outputs
    // - digit 1: 2 segments
    // - digit 4: 4 segments
    // - digit 7: 3 segments
    // - digit 8: 7 segments
    let mut count: u32 = 0;
    for entry in &entries {
        for digit in &entry.output.0 {
            if matches!(digit.len(), 2 | 4 | 3 | 7) {
                count += 1;
            }
        }
    }
    println!("Part 1: {}", count);

    // Part 2: actually do full matching based on signal-output pair SAT solving
    let displays: Vec<Display> = entries.iter().map(|entry| entry.solve()).collect();
    let sum: u32 = displays
        .iter()
        .map(|display| {
            let d: u32 = display.clone().into();
            d
        })
        .sum();
    println!("Part 2: {}", sum);
}

#[derive(Debug, Clone)]
struct Signal(Vec<String>); // TODO: [String; 10]
#[derive(Debug, Clone)]
struct ScrambledDisplay(Vec<String>); // TODO: [String; 4]

#[derive(Debug, Clone)]
struct Entry {
    signal: Signal,
    output: ScrambledDisplay,
}

impl Entry {
    fn new(signal: &str, output: &str) -> Self {
        Self {
            signal: Signal {
                0: signal
                    .split_ascii_whitespace()
                    .map(|signal| signal.to_owned())
                    .collect(),
            },
            output: ScrambledDisplay {
                0: output
                    .split_ascii_whitespace()
                    .map(|digit| digit.to_owned())
                    .collect(),
            },
        }
    }

    fn solve(&self) -> Display {
        // first let's split the four scrambled digits
        let scrambled_0 = &self.output.0[0];
        let scrambled_1 = &self.output.0[1];
        let scrambled_2 = &self.output.0[2];
        let scrambled_3 = &self.output.0[3];

        todo!()
    }
}

#[derive(Debug, Clone)]
struct Digit {
    segments: HashSet<char>,
}

impl Digit {
    fn new(segments: HashSet<char>) -> Self {
        Self { segments }
    }
}

impl FromStr for Digit {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        let chars = s.chars();
        let mut segments = HashSet::new();
        for char in chars {
            match char {
                'a' | 'b' | 'c' | 'd' | 'e' | 'f' | 'g' => segments.insert(char),
                _ => {
                    return Err(anyhow!(
                        "Invalid character {} in segment input for digit",
                        char,
                    ))
                }
            };
        }
        Ok(Self { segments })
    }
}

impl From<Digit> for u8 {
    fn from(val: Digit) -> Self {
        let segments_vec: Vec<char> = val.segments.into_iter().collect();
        match segments_vec[..] {
            ['a', 'b', 'c', 'e', 'f', 'g'] => 0,
            ['c', 'f'] => 1,
            ['a', 'c', 'd', 'e', 'g'] => 2,
            ['a', 'c', 'd', 'f', 'g'] => 3,
            ['b', 'c', 'd', 'f'] => 4,
            ['a', 'b', 'd', 'f', 'g'] => 5,
            ['a', 'b', 'd', 'e', 'f', 'g'] => 6,
            ['a', 'c', 'f'] => 7,
            ['a', 'b', 'c', 'd', 'e', 'f', 'g'] => 8,
            ['a', 'b', 'c', 'd', 'f', 'g'] => 9,

            _ => unimplemented!(),
        }
    }
}

#[derive(Debug, Clone)]
struct Display(Digit, Digit, Digit, Digit);

impl From<Display> for u32 {
    fn from(d: Display) -> Self {
        // TODO: this feels unwieldy
        let d0: u8 = d.0.into();
        let d1: u8 = d.1.into();
        let d2: u8 = d.2.into();
        let d3: u8 = d.3.into();

        1000 * d0 as u32 + 100 * d1 as u32 + 10 * d2 as u32 + d3 as u32
    }
}
