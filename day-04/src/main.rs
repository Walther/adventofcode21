use ansi_term::{Colour::Green, Style};
use std::fmt::Display;
use std::str::FromStr;

use anyhow::{Error, Result};
fn main() -> Result<()> {
    const INPUT: &str = include_str!("input.txt");

    // Different parts of the input file are separated by two newlines
    let mut parts = INPUT.split("\n\n");

    // First line has the numbers that will get called in order
    let numbers: Vec<u8> = parts
        .next()
        .unwrap()
        .split(',')
        .map(|n| n.parse().unwrap())
        .collect();

    // Collect all Bingo boards
    let mut boards: Vec<Board> = parts
        .skip(1)
        .map(|board| board.parse())
        .collect::<Result<_, _>>()?;

    // Read out numbers, mark the boards
    let mut first_win = false;

    // TODO: use a builtin .iter_mut and .iter, if you can figure out how to avoid the mutable+immutable borrow issue. See below.
    let num_boards = boards.len();
    'bingo: for number in numbers {
        for board in 0..num_boards {
            boards[board].mark(number);

            // Did we get our first win?
            if !first_win && boards[board].bingo() {
                first_win = true;
                println!("Part 1: {}", boards[board].score() * number as u32);
            }

            // Did we get our last win?
            if boards.iter().all(|board| board.bingo()) {
                println!("Part 2: {}", boards[board].score() * number as u32);
                break 'bingo;
            }
        }
    }

    // 'bingo: for number in numbers {
    //     for board in boards.iter_mut() {
    //         board.mark(number);
    //         // Did we get our first win?
    //         if !first_win && board.bingo() {
    //             first_win = true;
    //             println!("Part 1: {}", board.score() * number as u32);
    //         }

    //         // Did we get our last win?
    //         if boards.iter().all(|board| board.bingo()) {
    //             // Error: cannot borrow `boards` as immutable because it is also borrowed as mutable
    //             println!("Part 2: {}", board.score() * number as u32);
    //             break 'bingo;
    //         }
    //     }
    // }

    Ok(())
}

#[derive(Debug, Clone)]
struct Square {
    value: u8,
    marked: bool,
}

impl Display for Square {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let output = format!("{:>2}", self.value);
        if self.marked {
            write!(f, "{}", Style::new().fg(Green).bold().paint(output))
        } else {
            write!(f, "{}", output)
        }
    }
}

impl Square {
    fn new(value: u8) -> Self {
        Self {
            value,
            marked: false,
        }
    }

    fn mark(&mut self) {
        self.marked = true;
    }
}

#[derive(Debug, Clone)]
struct Board {
    squares: Vec<Vec<Square>>,
}

impl Board {
    fn mark(&mut self, number: u8) {
        self.squares.iter_mut().flatten().for_each(|square| {
            if square.value == number {
                square.mark();
            }
        });
    }

    fn bingo(&self) -> bool {
        // Check all rows
        for row in self.squares.iter() {
            if row.iter().all(|square| square.marked) {
                return true;
            }
        }

        // Check all columns
        for column in 0..5 {
            if self.squares.iter().all(|row| row[column].marked) {
                return true;
            }
        }

        false
    }

    fn score(&self) -> u32 {
        let unmarked = self
            .squares
            .iter()
            .flatten()
            .filter(|square| !square.marked);
        unmarked.map(|square| square.value as u32).sum()
    }
}

impl FromStr for Board {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut squares: Vec<Vec<Square>> = Vec::new();
        let lines: Vec<_> = s.split('\n').collect();
        for line in lines {
            let numbers: Vec<u8> = line
                .trim()
                .split_whitespace()
                .map(|n| n.parse())
                .collect::<Result<_, _>>()?;
            let mut row = Vec::new();
            for number in numbers {
                let square = Square::new(number);
                row.push(square);
            }
            squares.push(row);
        }

        Ok(Board { squares })
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        let mut output = String::new();
        for row in &self.squares {
            let row: Vec<String> = row.iter().map(|square| square.to_string()).collect();
            output.push_str(&row.join(" "));

            output.push('\n');
        }
        write!(f, "{}", output)
    }
}
