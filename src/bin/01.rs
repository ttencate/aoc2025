fn run(input: &str) -> (u64, u64) {
    let mut password_1 = 0;
    let mut password_2 = 0;
    let mut dial = 50_u64;
    for line in input.lines() {
        let [head, tail @ ..] = line.as_bytes() else {
            panic!("{line}");
        };
        let left = match head {
            b'L' => true,
            b'R' => false,
            other => panic!("{other}"),
        };
        if left {
            dial = (100 - dial).rem_euclid(100);
        }
        dial += std::str::from_utf8(tail).unwrap().parse::<u64>().unwrap();
        password_2 += dial / 100;
        dial = dial.rem_euclid(100);
        if dial == 0 {
            password_1 += 1;
        }
        if left {
            dial = (100 - dial).rem_euclid(100);
        }
    }
    (password_1, password_2)
}

aoc::main!(run);

#[test]
fn part_1() {
    assert_eq!(run("R50").0, 1);
    assert_eq!(run("L50").0, 1);
    assert_eq!(run("L50\nR100").0, 2);
    assert_eq!(run("L50\nL100").0, 2);
    assert_eq!(run("L50\nR199").0, 1);
    assert_eq!(run("L50\nL199").0, 1);
    assert_eq!(run("L50\nR200").0, 2);
    assert_eq!(run("L50\nL200").0, 2);
}

#[test]
fn part_2() {
    assert_eq!(run("R1000").1, 10);
    assert_eq!(run("R50").1, 1);
    assert_eq!(run("L50").1, 1);
    assert_eq!(run("L50\nR100").1, 2);
    assert_eq!(run("L50\nL100").1, 2);
    assert_eq!(run("L50\nR199").1, 2);
    assert_eq!(run("L50\nL199").1, 2);
    assert_eq!(run("L50\nR200").1, 3);
    assert_eq!(run("L50\nL200").1, 3);
}

#[test]
fn example() {
    let input = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82
";
    assert_eq!(run(input), (3, 6));
}
