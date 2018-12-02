use std::collections::HashSet;
use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<Error>> {
    let input = fs::read_to_string("input/01.txt")?;

    // part 1
    let part_1 = input.lines()
        .map(|line| {
            line
                .parse::<i64>()
                .expect("couldn't parse to int")
        })
        .sum::<i64>();

    println!("Part 1: {}", part_1);

    // part 2
    let mut current_freq = 0;
    let mut freq_reached = HashSet::new();
    freq_reached.insert(0);

    'outer: loop {
        for line in input.lines() {
            let n: i64 = line.parse()?;

            current_freq += n;

            if !freq_reached.insert(current_freq) {
                break 'outer;
            }
        }
    }

    println!("Part 2: {}", current_freq);

    Ok(())
}
