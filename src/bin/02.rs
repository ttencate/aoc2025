fn run(input: &str) -> (u64, u64) {
    let mut part_1 = 0;
    let mut part_2 = 0;
    for range in input.split(',') {
        let [fst, lst] = range
            .trim()
            .split('-')
            .map(|s| s.parse::<u64>().unwrap())
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        for id in fst..=lst {
            if is_invalid_1(id) {
                part_1 += id;
            }
            if is_invalid_2(id) {
                part_2 += id;
            }
        }
    }
    (part_1, part_2)
}

fn is_invalid_1(id: u64) -> bool {
    if id == 0 {
        return false;
    }
    let num_digits = id.ilog10() + 1;
    if !num_digits.is_multiple_of(2) {
        return false;
    }
    let pow = 10_u64.pow(num_digits / 2);
    let fst = id / pow;
    let snd = id % pow;
    fst == snd
}

#[test]
fn test_is_invalid_1() {
    assert!(!is_invalid_1(0));
    assert!(!is_invalid_1(9));
    assert!(!is_invalid_1(10));
    assert!(is_invalid_1(11));
    assert!(!is_invalid_1(12));
    assert!(!is_invalid_1(121));
    assert!(is_invalid_1(1010));
    assert!(is_invalid_1(1188511885));
}

#[test]
fn test_part_1() {
    assert_eq!(run(&aoc::example!(0)).0, 1227775554);
}

fn is_invalid_2(id: u64) -> bool {
    if id == 0 {
        return false;
    }
    let num_digits = id.ilog10() + 1;
    'outer: for splits in 2..=num_digits {
        if !num_digits.is_multiple_of(splits) {
            continue;
        }
        let pow = 10_u64.pow(num_digits / splits);
        let part = id % pow;
        let mut remaining = id / pow;
        while remaining != 0 {
            if remaining % pow != part {
                continue 'outer;
            }
            remaining /= pow;
        }
        return true;
    }
    false
}

#[test]
fn test_is_invalid_2() {
    assert!(!is_invalid_2(0));
    assert!(!is_invalid_2(9));
    assert!(!is_invalid_2(10));
    assert!(is_invalid_2(11));
    assert!(!is_invalid_2(12));
    assert!(!is_invalid_2(121));
    assert!(is_invalid_2(1010));
    assert!(is_invalid_2(222222));
    assert!(is_invalid_2(565656));
    assert!(is_invalid_2(1188511885));
    assert!(is_invalid_2(2121212121));
}

#[test]
fn test_part_2() {
    assert_eq!(run(&aoc::example!(0)).1, 4174379265);
}

aoc::main!(run);
