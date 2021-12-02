use anyhow::{anyhow, Context, Error, Result};
use std::str::FromStr;

fn main() -> Result<()> {
    const INPUT: &str = include_str!("input.txt");
    let lines = INPUT.lines();

    // We have two travel modes for the same instructions:
    // - simple, literal travel based on the instruction direction and offset
    // - aim, where up and down instructions change the tilt of the submarine, and only forward instruction is used for travel
    // We go through the instruction list once and track our travel based on both methods at the same time.
    let mut simple_coords: Vec<(u32, u32)> = Vec::new();
    let mut aim_coords: Vec<(u32, u32)> = Vec::new();
    let mut simple_position = (0, 0);
    let mut aim_position = (0, 0);
    let mut aim = 0;
    simple_coords.push(simple_position);
    aim_coords.push(aim_position);

    for line in lines {
        let instruction: Instruction = line.parse()?;

        // NOTE: Submarine coordinates: origin is (0,0), y down is positive
        match instruction {
            Instruction::Forward(offset) => {
                simple_position.0 += offset;
                aim_position.0 += offset;
                aim_position.1 += aim * offset;
            }
            Instruction::Down(offset) => {
                simple_position.1 += offset;
                aim += offset;
            }
            Instruction::Up(offset) => {
                simple_position.1 -= offset;
                aim -= offset;
            }
        }
        simple_coords.push(simple_position);
        aim_coords.push(aim_position);
    }

    // final x and y coordinates multiplied
    let simple_final = simple_coords.last().unwrap();
    println!("Part 1: {}", simple_final.0 * simple_final.1);
    let aim_final = aim_coords.last().unwrap();
    println!("Part 2: {}", aim_final.0 * aim_final.1);
    Ok(())
}

#[derive(Debug)]
enum Instruction {
    Forward(u32),
    Down(u32),
    Up(u32),
}

impl FromStr for Instruction {
    type Err = Error;
    fn from_str(line: &str) -> Result<Instruction> {
        let (direction, offset) = line
            .split_once(' ')
            .context("Failed to split instruction to direction and offset")?;
        let offset: u32 = offset.parse()?;

        match direction {
            "forward" => Ok(Instruction::Forward(offset)),
            "down" => Ok(Instruction::Down(offset)),
            "up" => Ok(Instruction::Up(offset)),
            _ => Err(anyhow!("Invalid direction")),
        }
    }
}
