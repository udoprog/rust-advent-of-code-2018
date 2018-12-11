use aoc2018::*;

fn part1(grid: &HashMap<(i64, i64), i64>) -> Option<(i64, i64, i64)> {
    let mut levels = Vec::new();

    for y in 0..(300 - 3) {
        for x in 0..(300 - 3) {
            let mut total = 0;

            for yp in y..(y + 3) {
                for xp in x..(x + 3) {
                    if let Some(level) = grid.get(&(xp, yp)).cloned() {
                        total += level;
                    }
                }
            }

            levels.push((x, y, total));
        }
    }

    levels.into_iter().max_by(|a, b| a.2.cmp(&b.2))
}

fn part2(grid: &HashMap<(i64, i64), i64>) -> Option<(i64, i64, i64, i64)> {
    let mut dynamic = HashMap::<_, i64>::new();
    let mut levels = Vec::new();

    let get = move |x, y| {
        grid.get(&(x, y)).cloned().unwrap_or_default()
    };

    for i in 1..=300 {
        for y in 0..=(300 - i) {
            for x in 0..=(300 - i) {
                let total = dynamic.entry((x, y)).or_default();

                for yp in y..(y + i) {
                    *total += get(x + i - 1, yp)
                }

                for xp in x..(x + i - 1) {
                    *total += get(xp, y + i - 1)
                }

                levels.push((x, y, i, *total));
            }
        }
    }

    levels.into_iter().max_by(|a, b| a.3.cmp(&b.3))
}


fn main() -> Result<(), Error> {
    let grid_serial = 7165i64;
    let mut grid = HashMap::new();

    for y in 1..=300i64 {
        for x in 1..=300i64 {
            let rack_id = x + 10;
            let mut level = rack_id * y;
            level += grid_serial;
            level *= rack_id;
            level = level % 1000;
            level = (level / 100) % 10;
            level -= 5;
            grid.insert((x, y), level);
        }
    }

    assert_eq!(part1(&grid), Some((235, 20, 31)));
    assert_eq!(part2(&grid), Some((237, 223, 14, 83)));
    Ok(())
}
