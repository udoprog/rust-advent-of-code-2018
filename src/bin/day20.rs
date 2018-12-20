use aoc2018::*;

use std::fmt;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Pos {
    x: i64,
    y: i64,
}

impl Pos {
    pub fn new(x: i64, y: i64) -> Pos {
        Pos { x, y }
    }

    fn step(self, dir: Dir) -> Pos {
        let Pos { x, y } = self;

        match dir {
            Dir::North => Pos::new(x, y - 1),
            Dir::East => Pos::new(x + 1, y),
            Dir::South => Pos::new(x, y + 1),
            Dir::West => Pos::new(x - 1, y),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Dir {
    North,
    East,
    South,
    West,
}

impl Dir {
    /// Reflect the direction into it's inverse.
    fn reflect(self) -> Dir {
        match self {
            Dir::North => Dir::South,
            Dir::East => Dir::West,
            Dir::South => Dir::North,
            Dir::West => Dir::East,
        }
    }
}

impl fmt::Display for Dir {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        let n = match *self {
            Dir::North => "N",
            Dir::East => "E",
            Dir::South => "S",
            Dir::West => "W",
        };

        n.fmt(fmt)
    }
}

#[derive(Debug, Clone)]
struct Expr {
    items: Vec<Item>,
}

impl fmt::Display for Expr {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        for item in &self.items {
            write!(fmt, "{}", item)?;
        }

        Ok(())
    }
}

#[derive(Debug, Clone)]
enum Item {
    Group(Vec<Expr>),
    Route(Vec<Dir>),
}

impl fmt::Display for Item {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Item::Group(ref entries) => {
                let mut it = entries.iter().peekable();

                write!(fmt, "(")?;

                while let Some(g) = it.next() {
                    fmt::Display::fmt(g, fmt)?;

                    if it.peek().is_some() {
                        write!(fmt, "|")?;
                    }
                }

                write!(fmt, ")")?;
            }
            Item::Route(ref route) => {
                for d in route {
                    d.fmt(fmt)?;
                }
            }
        }

        Ok(())
    }
}

impl Expr {
    pub fn parse(input: &str) -> Result<Expr, Error> {
        use std::mem;

        let mut route = Vec::new();
        let mut items = Vec::new();

        let mut it = input.chars();

        while let Some(c) = it.next() {
            match c {
                '^' | '$' => {}
                '(' => {
                    if !route.is_empty() {
                        items.push(Item::Route(mem::replace(&mut route, Vec::new())));
                    }

                    items.push(Item::Group(Self::parse_group(&mut it)?));
                }
                'N' => route.push(Dir::North),
                'E' => route.push(Dir::East),
                'S' => route.push(Dir::South),
                'W' => route.push(Dir::West),
                c => {
                    bail!("bad character in input: {}", c);
                }
            }
        }

        if !route.is_empty() {
            items.push(Item::Route(mem::replace(&mut route, Vec::new())));
        }

        Ok(Expr { items })
    }

    fn parse_group(it: &mut Iterator<Item = char>) -> Result<Vec<Expr>, Error> {
        use std::mem;

        let mut route = Vec::new();
        let mut items = Vec::new();
        let mut entries = Vec::new();

        while let Some(c) = it.next() {
            match c {
                '|' => {
                    if !route.is_empty() {
                        items.push(Item::Route(mem::replace(&mut route, Vec::new())));
                    }

                    entries.push(Expr {
                        items: mem::replace(&mut items, Vec::new()),
                    })
                }
                '(' => {
                    if !route.is_empty() {
                        items.push(Item::Route(mem::replace(&mut route, Vec::new())));
                    }

                    items.push(Item::Group(Self::parse_group(it)?));
                }
                'N' => route.push(Dir::North),
                'E' => route.push(Dir::East),
                'S' => route.push(Dir::South),
                'W' => route.push(Dir::West),
                ')' => {
                    if !route.is_empty() {
                        items.push(Item::Route(route));
                    }

                    entries.push(Expr { items });

                    return Ok(entries);
                }
                c => {
                    bail!("bad character in input: {}", c);
                }
            }
        }

        bail!("missing closing parenthesis")
    }

    pub fn walk(&self) -> Result<HashMap<Pos, HashSet<Dir>>, Error> {
        let mut doors = HashMap::<Pos, HashSet<Dir>>::new();
        self.walk_inner(Pos::default(), &mut doors)?;
        Ok(doors)
    }

    pub fn walk_inner(
        &self,
        pos: Pos,
        doors: &mut HashMap<Pos, HashSet<Dir>>,
    ) -> Result<HashSet<Pos>, Error> {
        let mut queue = VecDeque::new();

        if let Some((item, items)) = self.items.split_first() {
            queue.push_back((pos, Some(item), items));
        }

        let mut positions = HashSet::new();

        while let Some((pos, item, items)) = queue.pop_front() {
            let item = match item {
                None => {
                    positions.insert(pos);
                    continue;
                }
                Some(item) => item,
            };

            match *item {
                Item::Route(ref route) => {
                    let mut pos = pos;

                    for d in route.iter().cloned() {
                        let n = pos.step(d);

                        doors.entry(pos).or_default().insert(d);
                        doors.entry(n).or_default().insert(d.reflect());

                        pos = n;
                    }

                    if let Some((item, items)) = items.split_first() {
                        queue.push_back((pos, Some(item), items));
                    } else {
                        queue.push_back((pos, None, items));
                    }
                }
                Item::Group(ref group) => {
                    let mut positions = HashSet::new();

                    for expr in group {
                        positions.extend(expr.walk_inner(pos, doors)?);
                    }

                    for pos in positions {
                        if let Some((item, items)) = items.split_first() {
                            queue.push_back((pos, Some(item), items));
                        } else {
                            queue.push_back((pos, None, items));
                        }
                    }
                }
            }
        }

        Ok(positions)
    }
}

/// Just for fun function to render a set of doors.
fn render(doors: &HashMap<Pos, HashSet<Dir>>) -> Result<(), Error> {
    let (mn, mx) = doors
        .keys()
        .cloned()
        .minmax()
        .into_option()
        .expect("min/max");

    let width = (mx.x - mn.x) as usize * 2 + 1;
    let height = (mx.y - mn.y) as usize * 2 + 1;

    let mut grid = HashMap::new();

    for y in mn.y..=mx.y {
        for x in mn.x..=mx.x {
            let px = (x - mn.x) as usize * 2 + 1;
            let py = (y - mn.y) as usize * 2 + 1;

            grid.insert((px, py), '.');

            let pos = Pos::new(x, y);

            for d in doors.get(&pos).into_iter().flat_map(|d| d.iter()).cloned() {
                let ((gx, gy), door) = match d {
                    Dir::North => ((px, py - 1), '-'),
                    Dir::East => ((px + 1, py), '|'),
                    Dir::South => ((px, py + 1), '-'),
                    Dir::West => ((px - 1, py), '|'),
                };

                if let Some(existing) = grid.insert((gx, gy), door) {
                    if existing != door {
                        bail!("existing `{}` != inserted `{}`", existing, door);
                    }
                }
            }
        }
    }

    for y in 0..=(width + 1) {
        for x in 0..=(height + 1) {
            match grid.get(&(x, y)) {
                Some(c) => print!("{}", c),
                None => print!("#"),
            }
        }

        println!("");
    }

    Ok(())
}

fn find_furthest(doors: &HashMap<Pos, HashSet<Dir>>) -> Option<usize> {
    let mut dist = HashMap::new();

    let mut queue = VecDeque::new();
    queue.push_back((Pos::default(), 0));

    while let Some((pos, d)) = queue.pop_front() {
        match dist.entry(pos) {
            hash_map::Entry::Vacant(e) => {
                e.insert(d);
            }
            hash_map::Entry::Occupied(mut e) => {
                if d >= *e.get() {
                    continue;
                }

                e.insert(d);
            }
        }

        for dir in doors.get(&pos).into_iter().flat_map(|d| d.iter()).cloned() {
            queue.push_back((pos.step(dir), d + 1));
        }
    }

    dist.values().max().cloned()
}

fn count_by_limit(doors: &HashMap<Pos, HashSet<Dir>>, limit: usize) -> usize {
    let mut dist = HashMap::new();

    let mut queue = VecDeque::new();
    queue.push_back((Pos::default(), 0));

    while let Some((pos, d)) = queue.pop_front() {
        match dist.entry(pos) {
            hash_map::Entry::Vacant(e) => {
                e.insert(d);
            }
            hash_map::Entry::Occupied(mut e) => {
                if d >= *e.get() {
                    continue;
                }

                e.insert(d);
            }
        }

        for dir in doors.get(&pos).into_iter().flat_map(|d| d.iter()).cloned() {
            queue.push_back((pos.step(dir), d + 1));
        }
    }

    dist.values().cloned().filter(move |d| *d >= limit).count()
}

fn part1(expr: Expr) -> Result<Option<usize>, Error> {
    let doors = expr.walk()?;
    render(&doors)?;
    Ok(find_furthest(&doors))
}

fn part2(expr: Expr) -> Result<usize, Error> {
    let doors = expr.walk()?;
    Ok(count_by_limit(&doors, 1000))
}

fn main() -> Result<(), Error> {
    assert_eq!(
        part1(Expr::parse(input_str!("day20a.txt").trim())?)?,
        Some(23)
    );
    assert_eq!(
        part1(Expr::parse(input_str!("day20b.txt").trim())?)?,
        Some(31)
    );
    assert_eq!(
        part1(Expr::parse(input_str!("day20.txt").trim())?)?,
        Some(3476)
    );
    assert_eq!(part2(Expr::parse(input_str!("day20.txt").trim())?)?, 8514);
    Ok(())
}
