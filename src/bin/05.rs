fn run(input: &str) -> (u64, u64) {
    let mut counting = false;
    let mut fresh_ranges = Vec::new();
    let mut part_1 = 0;
    for line in input.lines() {
        if line.is_empty() {
            counting = true;
        } else if !counting {
            let mut parts = line.split('-');
            let from = parts.next().unwrap().parse::<u64>().unwrap();
            let to = parts.next().unwrap().parse::<u64>().unwrap();
            fresh_ranges.push(from..=to);
        } else {
            let id = line.parse::<u64>().unwrap();
            if fresh_ranges.iter().any(|r| r.contains(&id)) {
                part_1 += 1;
            }
        }
    }

    let mut part_2 = 0;
    let mut events = fresh_ranges
        .into_iter()
        .flat_map(|r| [(*r.start(), 1), (*r.end() + 1, -1)])
        .collect::<Vec<_>>();
    events.sort_unstable();
    let mut id = 0;
    let mut in_ranges = 0;
    for (at, delta) in events {
        if in_ranges > 0 {
            part_2 += at - id;
        }
        id = at;
        in_ranges += delta;
    }

    (part_1, part_2)
}

#[test]
fn test_part_1() {
    assert_eq!(run(&aoc::example!(0)).0, 3);
}

#[test]
fn test_part_2() {
    assert_eq!(run(&aoc::example!(0)).1, 14);
}

aoc::main!(run);
