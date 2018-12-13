use aoc2018::*;

#[derive(Clone, Copy, Debug)]
enum Area {
    Track,
    Inter,
    Slash,
    BackSlash,
}

#[derive(Clone, Copy, Debug)]
enum Dir {
    Right,
    Left,
    Up,
    Down,
}

impl Dir {
    fn apply(&mut self, g: Area, turn: &mut Turn) {
        *self = match (*self, g) {
            (_, Area::Track) => return,
            (dir, Area::Inter) => turn.apply(dir),
            (Dir::Left, Area::Slash) => Dir::Down,
            (Dir::Left, Area::BackSlash) => Dir::Up,
            (Dir::Right, Area::Slash) => Dir::Up,
            (Dir::Right, Area::BackSlash) => Dir::Down,
            (Dir::Up, Area::Slash) => Dir::Right,
            (Dir::Up, Area::BackSlash) => Dir::Left,
            (Dir::Down, Area::Slash) => Dir::Left,
            (Dir::Down, Area::BackSlash) => Dir::Right,
        };
    }
}

#[derive(Clone, Copy, Debug)]
enum Turn {
    Left,
    Straight,
    Right,
}

type Cart = ((i64, i64), Turn, Dir);

impl Turn {
    fn apply(&mut self, cart: Dir) -> Dir {
        let out = match (*self, cart) {
            (Turn::Left, Dir::Up) => Dir::Left,
            (Turn::Left, Dir::Left) => Dir::Down,
            (Turn::Left, Dir::Down) => Dir::Right,
            (Turn::Left, Dir::Right) => Dir::Up,

            (Turn::Straight, cart) => cart,

            (Turn::Right, Dir::Up) => Dir::Right,
            (Turn::Right, Dir::Right) => Dir::Down,
            (Turn::Right, Dir::Down) => Dir::Left,
            (Turn::Right, Dir::Left) => Dir::Up,
        };

        *self = match *self {
            Turn::Left => Turn::Straight,
            Turn::Straight => Turn::Right,
            Turn::Right => Turn::Left,
        };

        out
    }
}

fn solve(first: bool, grid: &HashMap<(i64, i64), Area>, mut carts: Vec<Cart>) -> (i64, i64) {
    loop {
        if carts.len() == 1 {
            return carts.into_iter().next().unwrap().0;
        }

        let mut positions = HashSet::new();
        let mut remove = HashSet::new();

        carts.sort_by(|a, b| {
            let (x0, y0) = a.0;
            let (x1, y1) = b.0;
            (y0, x0).cmp(&(y1, x1))
        });

        for (pos, _, _) in &mut carts {
            if !positions.insert(pos.clone()) {
                if first {
                    return *pos;
                }

                remove.insert(*pos);
            }
        }

        // no crashes, run simulation.

        for (_, (ref mut pos, ref mut turn, ref mut dir)) in carts.iter_mut().enumerate() {
            if remove.contains(pos) {
                continue;
            }

            positions.remove(pos);

            match *dir {
                Dir::Left => pos.0 -= 1,
                Dir::Right => pos.0 += 1,
                Dir::Up => pos.1 -= 1,
                Dir::Down => pos.1 += 1,
            }

            if !positions.insert(*pos) {
                if first {
                    return *pos;
                }

                remove.insert(*pos);
                continue;
            }

            let g = match grid.get(pos).cloned() {
                Some(g) => g,
                None => panic!("nothing on grid: {:?}", pos),
            };

            dir.apply(g, turn);
        }

        if !remove.is_empty() {
            carts = carts
                .into_iter()
                .filter(|c| !remove.contains(&c.0))
                .collect();
        }
    }
}

fn main() -> Result<(), Error> {
    //let lines = lines!(input!("day13.txt"), u32).collect::<Result<Vec<_>, _>>()?;
    //let columns = columns!(input!("day13.txt"), char::is_whitespace, u32);
    let lines = input_str!("day13.txt").lines().collect::<Vec<_>>();

    let mut carts = Vec::new();
    let mut grid = HashMap::new();

    for (y, line) in lines.into_iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let (x, y) = (x as i64, y as i64);

            match c {
                '+' => {
                    grid.insert((x, y), Area::Inter);
                }
                '/' => {
                    grid.insert((x, y), Area::Slash);
                }
                '\\' => {
                    grid.insert((x, y), Area::BackSlash);
                }
                '-' | '|' => {
                    grid.insert((x, y), Area::Track);
                }
                '>' => {
                    carts.push(((x, y), Turn::Left, Dir::Right));
                    grid.insert((x, y), Area::Track);
                }
                '^' => {
                    carts.push(((x, y), Turn::Left, Dir::Up));
                    grid.insert((x, y), Area::Track);
                }
                '<' => {
                    carts.push(((x, y), Turn::Left, Dir::Left));
                    grid.insert((x, y), Area::Track);
                }
                'v' => {
                    carts.push(((x, y), Turn::Left, Dir::Down));
                    grid.insert((x, y), Area::Track);
                }
                ' ' => {}
                o => {
                    panic!("unsupported: {}", o);
                }
            }
        }
    }

    assert_eq!(solve(true, &grid, carts.clone()), (83, 49));
    assert_eq!(solve(false, &grid, carts.clone()), (73, 36));
    Ok(())
}
