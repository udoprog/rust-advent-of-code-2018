use aoc2018::*;

fn main() -> Result<(), Error> {
    use std::io::Cursor;

    let lines = input_str!("day10.txt").lines().collect::<Vec<_>>();

    let mut points = Vec::new();

    for line in lines {
        let cols = columns!(Cursor::new(line), |c| !char::is_numeric(c) && c != '-', i32);
        let pos = na::Vector2::new(cols[0], cols[1]);
        let vel = na::Vector2::new(cols[2], cols[3]);

        points.push((pos, vel));
    }

    for i in 1.. {
        let mut xp = MinMax::default();
        let mut yp = MinMax::default();

        for &mut (ref mut pos, ref vel) in &mut points {
            *pos += *vel;

            xp.sample(pos.x);
            yp.sample(pos.y);
        }

        if yp.delta() != Some(9) {
            continue;
        }

        let mut by_pos = HashSet::new();

        for &(ref pos, _) in &points {
            by_pos.insert(*pos);
        }

        for y in yp.range_inclusive() {
            for x in xp.range_inclusive() {
                if by_pos.contains(&na::Vector2::new(x, y)) {
                    print!("#");
                } else {
                    print!(" ");
                }
            }

            println!("");
        }

        println!("Part 2: {}", i);
        break;
    }

    Ok(())
}
