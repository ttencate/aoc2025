use std::str::FromStr;

#[derive(Copy, Clone, Debug)]
struct Pos(i64, i64, i64);

impl Pos {
    fn squared_dist(self, other: Pos) -> i64 {
        (self.0 - other.0).pow(2) + (self.1 - other.1).pow(2) + (self.2 - other.2).pow(2)
    }
}

impl FromStr for Pos {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(',');
        let x = parts.next().unwrap().parse::<i64>().unwrap();
        let y = parts.next().unwrap().parse::<i64>().unwrap();
        let z = parts.next().unwrap().parse::<i64>().unwrap();
        Ok(Self(x, y, z))
    }
}

struct State {
    boxes: Vec<Pos>,
    squared_dists: Vec<(usize, usize, i64)>,
    circuits: Vec<usize>,
}

impl State {
    fn new(input: &str) -> Self {
        let boxes = input
            .lines()
            .map(|line| line.parse::<Pos>().unwrap())
            .collect::<Vec<_>>();
        let n = boxes.len();

        let boxes_ref = &boxes;
        let mut squared_dists = (0..n)
            .flat_map(|i| (i + 1..n).map(move |j| (i, j, boxes_ref[i].squared_dist(boxes_ref[j]))))
            .collect::<Vec<_>>();
        squared_dists.sort_unstable_by_key(|(_, _, sq_d)| -sq_d);

        let circuits = (0..n).collect::<Vec<_>>();

        Self {
            boxes,
            circuits,
            squared_dists,
        }
    }

    fn len(&self) -> usize {
        self.boxes.len()
    }

    fn try_connect_next(&mut self) -> Option<(usize, usize)> {
        let (a, b, _) = self.squared_dists.pop().unwrap();
        let from = self.circuits[a];
        let to = self.circuits[b];
        if from != to {
            for c in &mut self.circuits {
                if *c == from {
                    *c = to;
                }
            }
            Some((a, b))
        } else {
            None
        }
    }
}

fn part_1(input: &str, num_connections: usize) -> u64 {
    let mut state = State::new(input);
    for _ in 0..num_connections {
        state.try_connect_next();
    }

    let mut circuit_sizes = vec![0; state.len()];
    for c in state.circuits {
        circuit_sizes[c] += 1;
    }
    circuit_sizes.sort_unstable_by_key(|&size| -(size as i64));
    circuit_sizes[..3].iter().product::<u64>()
}

fn part_2(input: &str) -> u64 {
    let mut state = State::new(input);
    let mut num_circuits = state.len();
    loop {
        if let Some((a, b)) = state.try_connect_next() {
            num_circuits -= 1;
            if num_circuits == 1 {
                return (state.boxes[a].0 * state.boxes[b].0) as u64;
            }
        }
    }
}

fn run(input: &str) -> (u64, u64) {
    (part_1(input, 1000), part_2(input))
}

#[test]
fn test_part_1() {
    assert_eq!(part_1(&aoc::example!(0), 10), 40);
}

#[test]
fn test_part_2() {
    assert_eq!(part_2(&aoc::example!(0)), 25272);
}

aoc::main!(run);
