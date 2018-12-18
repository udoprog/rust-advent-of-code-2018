use aoc2018::*;
use std::ops;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Pos {
    x: i64,
    y: i64,
}

impl Pos {
    pub fn new(x: i64, y: i64) -> Pos {
        Pos { x, y }
    }

    pub fn neighbours(&self) -> [Pos; 8] {
        let Pos { x, y } = *self;

        [
            Pos::new(x + 1, y - 1),
            Pos::new(x + 1, y),
            Pos::new(x + 1, y + 1),
            Pos::new(x, y + 1),
            Pos::new(x - 1, y + 1),
            Pos::new(x - 1, y),
            Pos::new(x - 1, y - 1),
            Pos::new(x, y - 1),
        ]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Tile {
    Wooded,
    Lumberyard,
    Open,
}

impl Tile {
    fn as_str(&self) -> &str {
        match *self {
            Tile::Wooded => "|",
            Tile::Lumberyard => "#",
            Tile::Open => ".",
        }
    }
}

#[derive(Debug)]
struct Grid {
    rx: ops::RangeInclusive<i64>,
    ry: ops::RangeInclusive<i64>,
    grid: HashMap<Pos, Tile>,
}

impl Grid {
    pub fn load(input: &str) -> Result<Grid, Error> {
        let mut grid = HashMap::new();

        let mut rx = MinMax::default();
        let mut ry = MinMax::default();

        for (y, line) in input.lines().enumerate() {
            let y = y as i64;
            ry.sample(y);

            for (x, c) in line.chars().enumerate() {
                let x = x as i64;
                rx.sample(x);

                let pos = Pos::new(x, y);

                grid.insert(
                    pos,
                    match c {
                        '#' => Tile::Lumberyard,
                        '|' => Tile::Wooded,
                        '.' => continue,
                        o => bail!("Unsupported tile: {}", o),
                    },
                );
            }
        }

        Ok(Grid {
            grid,
            rx: rx.range_inclusive(),
            ry: ry.range_inclusive(),
        })
    }

    pub fn get(&self, pos: Pos) -> Tile {
        self.grid.get(&pos).cloned().unwrap_or(Tile::Open)
    }

    pub fn result(&self) -> usize {
        let mut wooded = 0;
        let mut lumberyards = 0;

        for pos in self.coords() {
            match self.get(pos) {
                Tile::Wooded => wooded += 1,
                Tile::Lumberyard => lumberyards += 1,
                Tile::Open => {}
            }
        }

        wooded * lumberyards
    }

    pub fn run<V>(&mut self, mut visuals: V, count: usize) -> Result<usize, Error>
    where
        V: Visuals,
    {
        use rayon::prelude::*;

        let coords = self.coords().collect::<Vec<_>>();

        V::setup();
        visuals.draw(0, self)?;

        let mut seen = HashMap::new();
        let mut results = Vec::new();

        for iter in 1..=count {
            let result = coords
                .par_iter()
                .cloned()
                .map(|pos| {
                    use self::Tile::*;

                    let tile = self.get(pos);

                    let mut wooden = 0;
                    let mut lumberyards = 0;

                    for n in pos.neighbours().iter().cloned() {
                        match self.get(n) {
                            Wooded => wooden += 1,
                            Lumberyard => lumberyards += 1,
                            Open => {}
                        }
                    }

                    match tile {
                        Open if wooden >= 3 => (pos, Wooded),
                        Open => (pos, tile),
                        Wooded if lumberyards >= 3 => (pos, Lumberyard),
                        Wooded => (pos, tile),
                        Lumberyard if lumberyards >= 1 && wooden >= 1 => (pos, Lumberyard),
                        Lumberyard => (pos, Open),
                    }
                })
                .collect::<Vec<_>>();

            let grid = result.into_iter().collect::<HashMap<_, _>>();
            let grid_vec = grid.clone().into_iter().collect::<Vec<_>>();

            if let Some(prev) = seen.insert(grid_vec, iter) {
                let cycle_length = iter - prev;
                let offset = (count - iter) % cycle_length;

                match results.get(results.len() - cycle_length + offset) {
                    Some(result) => return Ok(*result),
                    None => bail!("result not found"),
                }
            }

            self.grid = grid;
            results.push(self.result());
            visuals.draw(iter, self)?;
        }

        V::teardown();
        Ok(self.result())
    }

    pub fn coords(&self) -> impl Iterator<Item = Pos> + '_ {
        self.ry
            .clone()
            .into_iter()
            .flat_map(move |y| self.rx.clone().into_iter().map(move |x| Pos::new(x, y)))
    }
}

fn main() -> Result<(), Error> {
    // Example
    assert_eq!(
        Grid::load(input_str!("day18a.txt"))?.run(NcursesVisuals::default(), 10)?,
        1147
    );
    // Part 1
    assert_eq!(
        Grid::load(input_str!("day18.txt"))?.run(NcursesVisuals::default(), 10)?,
        606416
    );
    // Part 2 (fast solution)
    assert_eq!(
        Grid::load(input_str!("day18.txt"))?.run(NoopVisuals, 1_000_000_000)?,
        210796
    );
    // Part 2 with nice visuals.
    assert_eq!(
        Grid::load(input_str!("day18.txt"))?.run(NcursesVisuals::default(), 1_000_000_000)?,
        210796
    );
    Ok(())
}

trait Visuals {
    fn setup();

    fn teardown();

    fn draw(&mut self, iter: usize, grid: &Grid) -> Result<(), Error>;
}

pub struct NoopVisuals;

impl Visuals for NoopVisuals {
    fn setup() {}

    fn teardown() {}

    fn draw(&mut self, iter: usize, grid: &Grid) -> Result<(), Error> {
        if iter % 1000 != 0 {
            return Ok(());
        }

        let mut wooded = 0;
        let mut lumberyards = 0;

        for pos in grid.coords() {
            match grid.get(pos) {
                Tile::Wooded => wooded += 1,
                Tile::Lumberyard => lumberyards += 1,
                Tile::Open => {}
            }
        }

        println!("iter: {}, {}", iter, wooded * lumberyards);
        Ok(())
    }
}

#[derive(Default)]
pub struct NcursesVisuals {
    /// Only visualize once every n frame.
    every: Option<usize>,
}

impl NcursesVisuals {
    pub fn every(mut self, frame: usize) -> Self {
        self.every = Some(frame);
        self
    }
}

impl Visuals for NcursesVisuals {
    fn setup() {
        use ncurses as n;

        n::initscr();
        n::noecho();
        n::curs_set(n::CURSOR_VISIBILITY::CURSOR_INVISIBLE);
    }

    fn teardown() {
        use ncurses as n;

        n::mv(0, 0);
        n::printw("press [enter] to exit...");

        loop {
            let c = n::getch();

            if c == 10 {
                break;
            }
        }

        n::endwin();
    }

    fn draw(&mut self, iter: usize, grid: &Grid) -> Result<(), Error> {
        use ncurses as n;

        if let Some(every) = self.every {
            n::mvprintw(0, 0, &format!("Iter: {}", iter));
            n::refresh();

            if iter % every != 0 {
                return Ok(());
            }
        } else {
            std::thread::sleep(std::time::Duration::from_millis(50));
        }

        n::erase();

        n::mvprintw(0, 0, &format!("Iter: {}", iter));

        for pos in grid.coords() {
            n::mv(pos.y as i32 + 1, pos.x as i32);
            n::printw(grid.get(pos).as_str());
        }

        n::refresh();
        Ok(())
    }
}
