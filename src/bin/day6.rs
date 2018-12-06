use aoc2018::*;

use std::fmt;

type Bounds = (i32, i32);
type Origin = usize;
type Coord = (i32, i32);

#[derive(Debug, Clone, Copy)]
enum Node {
    Conflicted(u32),
    Distance(Origin, u32),
}

impl fmt::Display for Node {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Node::Conflicted(_) => "..".fmt(fmt),
            Node::Distance(o, _) => write!(fmt, "{:02}", o),
        }
    }
}

/// Traverse and mark entire space of coordinates.
///
/// Use the arena bounds to determine the maximum distance to mark before we give up.
fn part1(bx: Bounds, by: Bounds, coords: &[Coord]) -> Option<u32> {
    let max_d = ((bx.1 - bx.0) + (by.1 - by.0)) as u32;

    let mut infinites = HashSet::new();
    let mut m = HashMap::new();

    for (i, c) in coords.iter().cloned().enumerate() {
        if !is_finite(c, &coords) {
            infinites.insert(i);
        }

        let mut queue = VecDeque::new();
        queue.push_back((c, 0));

        while let Some((c, d)) = queue.pop_front() {
            if d > max_d {
                continue;
            }

            let step = match m.entry(c) {
                hash_map::Entry::Vacant(e) => {
                    e.insert(Node::Distance(i, d));
                    true
                },
                hash_map::Entry::Occupied(mut e) => {
                    // test existing node.
                    match *e.get() {
                        Node::Distance(_, p) | Node::Conflicted(p) if p > d => {
                            e.insert(Node::Distance(i, d));
                            true
                        },
                        Node::Distance(other, p) if p == d && other != i => {
                            e.insert(Node::Conflicted(d));
                            false
                        },
                        _ => false,
                    }
                },
            };

            if step {
                queue.extend(neigh(c).into_iter().map(|c| (*c, d + 1)));
            }
        }
    }

    let mut results = HashMap::<usize, u32>::new();

    for y in by.0..=by.1 {
        for x in bx.0..=bx.1 {
            let c = (x, y);

            if let Some(Node::Distance(o, _)) = m.get(&c).cloned() {
                if !infinites.contains(&o) {
                    *results.entry(o).or_default() += 1;
                }
            }
        }
    }

    return results.into_iter().max_by(|a, b| a.1.cmp(&b.1)).map(|n| n.1);

    /// Test if a coord is constrained in all directions.
    ///
    /// A coord is constrained if any other coordinate would reach an intersection faster than the
    /// coordinate being tested in all directions.
    fn is_finite(c: Coord, coords: &[Coord]) -> bool {
        // various directions we might be constrained.
        let mut c_px = false;
        let mut c_nx = false;
        let mut c_py = false;
        let mut c_ny = false;

        for t in coords.iter().cloned() {
            if (t.0, t.1) == (c.0, c.1) {
                continue;
            }

            let dx = (t.0 - c.0).abs() as u32;
            let dy = (t.1 - c.1).abs() as u32;

            if dx >= dy {
                if t.0 > c.0 {
                    c_px = true;
                } else {
                    c_nx = true;
                }
            }

            if dy >= dx {
                if t.1 > c.1 {
                    c_py = true;
                } else {
                    c_ny = true;
                }
            }
        }

        c_px && c_nx && c_py && c_ny
    }
}

/// Find all coordinates that satisfy the given constraints.
///
/// We know that if one coordinate exists, it has to be within the bounds, so start looking there.
fn part2(bx: Bounds, by: Bounds, constraint: impl Fn(Coord) -> bool) -> usize {
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();

    for y in by.0..=by.1 {
        for x in bx.0..=bx.1 {
            queue.push_back((x, y));
        }
    }

    let mut count = 0;

    while let Some(c) = queue.pop_front() {
        if visited.contains(&c) {
            continue;
        }

        visited.insert(c);

        if constraint(c) {
            count += 1;
            queue.extend(neigh(c).into_iter());
        }
    }

    count
}

fn part2_constraint(c: Coord, coords: &[Coord]) -> bool {
    let mut sum = 0;

    for o in coords.iter().cloned() {
        sum += (c.0 - o.0).abs() + (c.1 - o.1).abs();
    }

    sum < 10000
}

fn main() -> Result<(), Error> {
    let mut bx = (None, None);
    let mut by = (None, None);

    let mut coords: Vec<Coord> = Vec::new();

    for line in lines!(input!("day6.txt"), Trim<i32>, i32) {
        let (Trim(x), y) = line?;

        bx.0 = min(bx.0, x);
        bx.1 = max(bx.1, x);
        by.0 = min(by.0, y);
        by.1 = max(by.1, y);

        coords.push((x, y));
    }

    let bx = match bx {
        (Some(s), Some(e)) => (s, e),
        _ => panic!("no x bounds"),
    };

    let by = match by {
        (Some(s), Some(e)) => (s, e),
        _ => panic!("no y bounds"),
    };


    assert_eq!(part1(bx, by, &coords), Some(3882));
    assert_eq!(part2(bx, by, |c| part2_constraint(c, &coords)), 43852);
    return Ok(());

    fn min(d: Option<i32>, n: i32) -> Option<i32> {
        match d {
            Some(d) if d < n => Some(d),
            _ => Some(n),
        }
    }

    fn max(d: Option<i32>, n: i32) -> Option<i32> {
        match d {
            Some(d) if d > n => Some(d),
            _ => Some(n),
        }
    }
}

/// Get all neighbours for the given node.
fn neigh((x, y): Coord) -> [Coord; 4] {
    [
        (x - 1, y),
        (x + 1, y),
        (x, y + 1),
        (x, y - 1),
    ]
}
