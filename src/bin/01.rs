use std::collections::HashSet;
use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<Error>> {
    let input = fs::read_to_string("input/01.txt")?;

    // part 1
    //
    // functional style
    // `::<>` is type annotation done "inline" with fn call, so that parse()
    // knows what type to parse to.
    //
    // In procedural section, the annotation is done with `let n: i64`
    // parse() returns a Result, and expect will panic if the result is
    // an Err.
    let part_1 = input.lines()
        .map(|line| {
            line
                .parse::<i64>()
                .expect("couldn't parse to int")
        })
        .sum::<i64>();

    println!("Part 1: {}", part_1);

    // part 2
    //
    // procedural style
    let mut current_freq = 0;
    let mut freq_reached = HashSet::new();
    freq_reached.insert(0);

    // loop is labeled to allow breaking two loops at once
    // I just tacked on the outer loop; if I knew I was going
    // to loop more than once, I'd parse to integer first.
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
