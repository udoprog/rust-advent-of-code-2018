#![feature(range_contains)]

use aoc2018::*;

use std::ops::RangeInclusive;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tile {
    Clay,
    Still,
    Flowing,
    OutOfBounds,
    None,
}

struct Tiles {
    source: (i64, i64),
    tiles: HashMap<(i64, i64), Tile>,
    ry: RangeInclusive<i64>,
    ry_with_source: RangeInclusive<i64>,
}

impl Tiles {
    pub fn load(input: &str) -> Tiles {
        let mut it = input.lines();
        let mut tiles = HashMap::new();

        // water source
        let source = (500, 0);

        let mut ry = MinMax::default();

        while let Some((x, y)) = parse(&mut it) {
            for x in x.0..=x.1 {
                for y in y.0..=y.1 {
                    tiles.insert((x, y), Tile::Clay);
                    ry.sample(y);
                }
            }
        }

        let mut ry_with_source = ry.clone();
        ry_with_source.sample(source.1);

        Tiles {
            source,
            tiles,
            ry: ry.range_inclusive(),
            ry_with_source: ry_with_source.range_inclusive(),
        }
    }

    /// Visualize the tiles.
    fn visualize(&self) {
        for y in self.ry.clone() {
            let (x0, x1) = self
                .tiles
                .iter()
                .map(|t| (t.0).0)
                .minmax()
                .into_option()
                .expect("minmax");

            for x in x0..=x1 {
                if (x, y) == self.source {
                    print!("+");
                    continue;
                }

                match self.get((x, y)) {
                    Tile::Clay => print!("#"),
                    Tile::Still => print!("~"),
                    Tile::Flowing => print!("|"),
                    Tile::None => print!("."),
                    _ => print!("?"),
                }
            }

            println!("");
        }
    }

    /// Check what is on the given tile.
    pub fn get(&self, (x, y): (i64, i64)) -> Tile {
        if !self.ry_with_source.contains(&y) {
            return Tile::OutOfBounds;
        }

        match self.tiles.get(&(x, y)).cloned() {
            Some(tile) => tile,
            None => Tile::None,
        }
    }

    /// Fill x range with something.
    pub fn fill_x(&mut self, x: RangeInclusive<i64>, y: i64, tile: Tile) -> Result<(), Error> {
        if !self.ry.contains(&y) {
            return Ok(());
        }

        for x in x {
            if let Some(existing) = self.tiles.insert((x, y), tile) {
                if existing != Tile::Flowing {
                    bail!("Already had thing `{:?}` at tile {:?}", existing, (x, y));
                }
            }
        }

        Ok(())
    }

    /// Fill y range with something.
    pub fn fill_y(&mut self, x: i64, y: RangeInclusive<i64>, tile: Tile) -> Result<(), Error> {
        for y in y {
            if !self.ry.contains(&y) {
                continue;
            }

            if let Some(existing) = self.tiles.insert((x, y), tile) {
                bail!("Already had thing `{:?}` at tile {:?}", existing, (x, y));
            }
        }

        Ok(())
    }
}

fn solve(tiles: &mut Tiles) -> Result<(usize, usize), Error> {
    // queue of water "drops"
    let mut drop_queue = VecDeque::new();
    drop_queue.push_back(tiles.source);

    let mut floor_queue = VecDeque::new();

    while !drop_queue.is_empty() || !floor_queue.is_empty() {
        while let Some((x, y)) = drop_queue.pop_front() {
            match scan_down(&tiles, (x, y)) {
                Some((pos, tile)) => {
                    tiles.fill_y(x, y..=pos.1, Tile::Flowing)?;

                    if tile != Tile::Flowing {
                        floor_queue.push_back(pos);
                    }
                }
                // NB: went out of bounds
                None => {
                    tiles.fill_y(x, y..=*tiles.ry.end(), Tile::Flowing)?;
                }
            }
        }

        // digest the floor queue.
        while let Some((x, y)) = floor_queue.pop_front() {
            // we are on a floor that is already filled, keep trying!
            if let Tile::Still = tiles.get((x, y)) {
                floor_queue.push_back((x, y - 1));
                continue;
            }

            let left = scan_floor(&tiles, (x, y), -1)?;
            let right = scan_floor(&tiles, (x, y), 1)?;

            match (left, right) {
                // bounded.
                ((Tile::Clay, left), (Tile::Clay, right)) => {
                    tiles.fill_x(left.0..=right.0, y, Tile::Still)?;
                    floor_queue.push_back((x, y - 1));
                }
                (left, right) => {
                    tiles.fill_x((left.1).0..=(right.1).0, y, Tile::Flowing)?;

                    for m in vec![left, right] {
                        match m {
                            (Tile::None, (x, y)) => {
                                drop_queue.push_back((x, y + 1));
                            }
                            (Tile::Clay, _) => {}
                            (Tile::OutOfBounds, _) => {}
                            (Tile::Flowing, _) => {}
                            other => {
                                bail!("Unexpected tile: {:?}", other);
                            }
                        }
                    }
                }
            }
        }
    }

    // NB: just to be safe, remove the source.
    tiles.tiles.remove(&tiles.source);

    let part1 = tiles
        .tiles
        .values()
        .cloned()
        .map(|t| match t {
            Tile::Flowing | Tile::Still => 1,
            _ => 0,
        })
        .sum();

    let part2 = tiles
        .tiles
        .values()
        .cloned()
        .map(|t| match t {
            Tile::Still => 1,
            _ => 0,
        })
        .sum();

    return Ok((part1, part2));

    /// Scan floor in some direction.
    ///
    /// Returns the coordinates and `None` if we hit a wall, the returned coordinates correspond to
    /// the last coordinates that had an open tile.
    ///
    /// Returns the open coordinate in case we no longer have a floor.
    /// The open coordinate is the coordinate at which the floor stopped.
    ///
    /// Otherwise, returns `None`.
    fn scan_floor(
        tiles: &Tiles,
        (mut x, y): (i64, i64),
        dir: i64,
    ) -> Result<(Tile, (i64, i64)), Error> {
        loop {
            match tiles.get((x + dir, y)) {
                Tile::Clay => return Ok((Tile::Clay, (x, y))),
                Tile::Still => bail!("Encountered unexpected still tile at {:?}", (x, y)),
                _ => {}
            }

            match tiles.get((x, y + 1)) {
                Tile::Clay | Tile::Still => {}
                tile => return Ok((tile, (x, y))),
            }

            x += dir;
        }
    }

    fn scan_down(tiles: &Tiles, (x, mut y): (i64, i64)) -> Option<((i64, i64), Tile)> {
        loop {
            match tiles.get((x, y)) {
                Tile::Flowing => return Some(((x, y - 1), Tile::Flowing)),
                Tile::None => {}
                Tile::OutOfBounds => return None,
                tile => return Some(((x, y - 1), tile)),
            }

            y += 1;
        }
    }
}

fn main() -> Result<(), Error> {
    assert_eq!(solve(&mut Tiles::load(input_str!("day17a.txt")))?, (57, 29));
    assert_eq!(
        solve(&mut Tiles::load(input_str!("day17.txt")))?,
        (34244, 28202)
    );
    Ok(())
}

fn parse<'a>(it: &mut impl Iterator<Item = &'a str>) -> Option<((i64, i64), (i64, i64))> {
    let line = it.next()?;
    let x = line.split(", ").nth(0)?;
    let x = str::parse(&x[2..]).ok()?;
    let x = (x, x);

    let y = line.split(", ").nth(1)?;

    let (is_y, y) = match y.split_at(2) {
        ("y=", rest) => (true, rest),
        (_, rest) => (false, rest),
    };

    let y = {
        let mut p = y.split("..");
        (str::parse(p.next()?).ok()?, str::parse(p.next()?).ok()?)
    };

    let (x, y) = if is_y { (x, y) } else { (y, x) };

    Some((x, y))
}
