fn run(input: &str) -> (u64, u64) {
    (
        input.lines().map(|bank| max_joltage(bank, 2)).sum(),
        input.lines().map(|bank| max_joltage(bank, 12)).sum(),
    )
}

fn max_joltage(bank: &str, num_digits: usize) -> u64 {
    let bank = bank
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u64)
        .collect::<Vec<_>>();
    // maxes[n] is the largest joltage we can make out of the current suffix
    // using exactly n digits. The suffix starts out empty.
    let mut maxes = vec![0; num_digits + 1];
    for (suffix_len, &digit) in bank.iter().rev().enumerate() {
        let len = suffix_len + 1;
        maxes = (0..=num_digits)
            .map(|n| {
                if n > 0 && n <= len {
                    let max_when_used = digit * 10_u64.pow(n as u32 - 1) + maxes[n - 1];
                    maxes[n].max(max_when_used)
                } else {
                    maxes[n]
                }
            })
            .collect();
    }

    maxes[num_digits]
}

#[test]
fn test_part_1() {
    assert_eq!(max_joltage("987654321111111", 2), 98);
    assert_eq!(max_joltage("811111111111119", 2), 89);
    assert_eq!(max_joltage("234234234234278", 2), 78);
    assert_eq!(max_joltage("818181911112111", 2), 92);
    assert_eq!(run(&aoc::example!(0)).0, 357);
}

#[test]
fn test_part_2() {
    assert_eq!(max_joltage("987654321111111", 12), 987654321111);
    assert_eq!(max_joltage("811111111111119", 12), 811111111119);
    assert_eq!(max_joltage("234234234234278", 12), 434234234278);
    assert_eq!(max_joltage("818181911112111", 12), 888911112111);
    assert_eq!(run(&aoc::example!(0)).1, 3121910778619);
}
aoc::main!(run);
