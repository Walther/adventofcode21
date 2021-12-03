use bitvec::prelude::*;

fn main() {
    const INPUT: &str = include_str!("input.txt");
    let lines = INPUT.lines();

    // Parsing phase. Parse into a 2D array of bits using a Vec of BitVecs
    // TODO: cleanup
    let mut table: Vec<BitVec> = Vec::new();
    for line in lines {
        let mut row = bitvec![];
        for bit in line.chars() {
            let value: u32 = bit.to_string().parse().unwrap();
            let value = match value {
                0 => false,
                1 => true,
                _ => panic!("Invalid bit deteced"),
            };
            row.push(value);
        }
        table.push(row);
    }
    let width = table[0].len();

    // Construct the values by collecting bits from the columns
    let mut gamma_bits = bitvec![];
    let mut epsilon_bits = bitvec![];

    for column in 0..width {
        gamma_bits.push(most_common_bit_in_column(&table, column));
        epsilon_bits.push(least_common_bit_in_column(&table, column));
    }

    // Bit order shenanigans. // TODO: cleanup!
    gamma_bits.reverse();
    epsilon_bits.reverse();

    // Convert bits into usize
    let gamma_rate = gamma_bits.into_vec()[0];
    let epsilon_rate = epsilon_bits.into_vec()[0];

    // Compute and print the result of Part 1
    let power_consumption = gamma_rate * epsilon_rate;
    println!("Part 1: {}", power_consumption);

    // Re-use the original data for new purposes
    let mut oxygen_rating = table.clone();
    let mut co2_rating = table.clone();

    // Iteratively filter through the tables to reduce the rows into one final row to keep

    for column in 0..width {
        let most_common = most_common_bit_in_column(&oxygen_rating, column);
        oxygen_rating.retain(|x| x[column] == most_common);
        if oxygen_rating.len() == 1 {
            break;
        }
    }

    for column in 0..width {
        let least_common = least_common_bit_in_column(&co2_rating, column);
        co2_rating.retain(|x| x[column] == least_common);
        if co2_rating.len() == 1 {
            break;
        }
    }

    // Get the bits of the final remaining rows // TODO: can this be simplified?
    let mut oxygen_bits = oxygen_rating[0].clone();
    let mut co2_bits = co2_rating[0].clone();

    // Bit order shenanigans. // TODO: cleanup!
    oxygen_bits.reverse();
    co2_bits.reverse();

    // Convert bits into usize
    let oxygen_rating = oxygen_bits.into_vec()[0];
    let co2_rating = co2_bits.into_vec()[0];

    // Compute and print the result of Part 2
    println!("Part 2: {}", oxygen_rating * co2_rating);
}

/// Given a 2D table of bits and a column index, returns the most common bit of that column. If there's an equal amount of 0s and 1s, 1 is returned.
fn most_common_bit_in_column(table: &[BitVec], column: usize) -> bool {
    let count: u32 = table.len() as u32;
    let mut sum = 0;
    for line in table {
        sum += line[column] as u32;
    }
    sum >= (count - sum)
}

/// Given a 2D table of bits and a column index, returns the least common bit of that column. If there's an equal amount of 0s and 1s, 0 is returned. This mirrors the behavior of [most_common_bit_in_column].
fn least_common_bit_in_column(table: &[BitVec], column: usize) -> bool {
    !most_common_bit_in_column(table, column)
}
