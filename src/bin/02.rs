fn main() -> Result< (), Box<std::error::Error>> {
    let input = std::fs::read_to_string("input/02.txt")?;

    let mut twos = 0;
    let mut threes = 0;

    for line in input.lines() {
        let counts = count_letters(line);

        if has_two_of(&counts) {
            twos += 1;
        }

        if has_three_of(&counts) {
            threes += 1;
        }
    }

    println!("Part 1 {}", twos * threes);

    Ok(())
}

// uses an array, with the indexes representing
// the letter of the alphabet, and the value at the
// index the count.
//
// The character is cast from an ascii char to its
// u8 (byte) value, then shifted by 97 so that 'a'
// starts at index 0.
fn count_letters(s: &str) -> Vec<u8> {
    let mut counts = vec![0;26];

    for c in s.chars() {
        counts[(c as u8 - 97) as usize] += 1;
    }
    counts
}

fn has_two_of(counts: &[u8]) -> bool {
    assert!(counts.len() == 26);

    // Find returns an Option(value that's find)
    // which I then convert to a bool by pattern
    // matching.
    match counts.iter().find(|n| **n == 2) {
        Some(_) => true,
        None => false,
    }
}

fn has_three_of(counts: &[u8]) -> bool {
    assert!(counts.len() == 26);

    match counts.iter().find(|n| **n == 3) {
        Some(_) => true,
        None => false,
    }
}
