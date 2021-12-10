use anyhow::{anyhow, Error, Result};
use std::str::FromStr;
use Token::*;
use Validity::*;

fn main() -> Result<()> {
    const INPUT: &str = include_str!("input.txt");

    // Input is a newline-separated list of code snippets
    // Each code snippet is treated as a separate entity
    // Each line consists of tokens {}[]()<>
    // We can categorize the lines into groups:
    // - valid, where every opening token has a matching closing one, and they are in correct order
    // - incomplete, where all tokens are in valid order, but some are left unclosed at the end of the line
    // - corrupted, where there are invalid closing tokens
    // - unknown, where we have not yet evaluated the line

    // Parse the lines into a list of Codes
    let mut codes: Vec<Code> = Vec::new();
    for line in INPUT.lines() {
        let mut code = Code::default();
        for char in line.chars() {
            let token = char.to_string().parse()?;
            code.tokens.push(token);
        }
        codes.push(code);
    }

    // Categorize each Code
    for code in codes.iter_mut() {
        code.validate();
    }

    // Go through the corrupted lines of code
    let mut syntax_error_high_score = 0;
    for code in codes.iter() {
        if let Corrupted(token) = &code.validity {
            // Increase the high score based on which token is corrupting the line
            match token {
                AngleRight => syntax_error_high_score += 25137,
                CurlyRight => syntax_error_high_score += 1197,
                RoundRight => syntax_error_high_score += 3,
                SquareRight => syntax_error_high_score += 57,
                _ => continue,
            }
        }
    }

    println!("Part 1: {}", syntax_error_high_score);

    // Go through the incomplete lines of code
    let mut autocomplete_high_scores: Vec<u64> = Vec::new();
    for code in codes.iter() {
        if let Incomplete(stack) = &code.validity {
            let mut high_score = 0;
            let mut stack = stack.clone();
            // Increase the score:
            // - multiply by 5 for each token
            // - add to the score based on the token that needs to be matched
            while let Some(token) = stack.pop() {
                high_score *= 5;
                match token {
                    AngleLeft => high_score += 4,
                    CurlyLeft => high_score += 3,
                    RoundLeft => high_score += 1,
                    SquareLeft => high_score += 2,
                    _ => continue,
                }
            }
            autocomplete_high_scores.push(high_score);
        }
    }
    // Print the middle value
    autocomplete_high_scores.sort_unstable();
    let middle = autocomplete_high_scores.len() / 2;
    println!("Part 2: {}", autocomplete_high_scores[middle]);

    Ok(())
}

#[derive(Debug, Clone, PartialEq)]
enum Token {
    AngleLeft,   // <
    AngleRight,  // >
    CurlyLeft,   // {
    CurlyRight,  // }
    RoundLeft,   // (
    RoundRight,  // )
    SquareLeft,  // [
    SquareRight, // ]
}

impl FromStr for Token {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "(" => Ok(RoundLeft),
            ")" => Ok(RoundRight),
            "[" => Ok(SquareLeft),
            "]" => Ok(SquareRight),
            "{" => Ok(CurlyLeft),
            "}" => Ok(CurlyRight),
            "<" => Ok(AngleLeft),
            ">" => Ok(AngleRight),
            _ => Err(anyhow!("Invalid token {}", s)),
        }
    }
}

#[derive(Default)]
struct Code {
    tokens: Vec<Token>,
    validity: Validity,
}

impl Code {
    fn validate(&mut self) {
        let mut stack = Vec::new();
        // First phase: closing ones must match the top of the stack
        for token in &self.tokens {
            match token {
                // Left
                AngleLeft => stack.push(token.to_owned()),
                CurlyLeft => stack.push(token.to_owned()),
                RoundLeft => stack.push(token.to_owned()),
                SquareLeft => stack.push(token.to_owned()),
                // Right
                AngleRight => {
                    let top = stack.pop().unwrap();
                    if top != AngleLeft {
                        self.validity = Corrupted(token.clone());
                        return;
                    }
                }
                CurlyRight => {
                    let top = stack.pop().unwrap();
                    if top != CurlyLeft {
                        self.validity = Corrupted(token.clone());
                        return;
                    }
                }
                RoundRight => {
                    let top = stack.pop().unwrap();
                    if top != RoundLeft {
                        self.validity = Corrupted(token.clone());
                        return;
                    }
                }
                SquareRight => {
                    let top = stack.pop().unwrap();
                    if top != SquareLeft {
                        self.validity = Corrupted(token.clone());
                        return;
                    }
                }
            }
        }
        // Second phase: do we have stack left?
        if stack.is_empty() {
            self.validity = Valid;
            return;
        }
        self.validity = Incomplete(stack.clone());
    }
}

#[derive(Debug, PartialEq)]
enum Validity {
    Corrupted(Token),
    Incomplete(Vec<Token>),
    Unknown,
    Valid,
}

impl Default for Validity {
    fn default() -> Self {
        Self::Unknown
    }
}
