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
        let mut xp = (1000000i32, -1000000i32);
        let mut yp = (1000000i32, -1000000i32);

        for &mut (ref mut pos, ref vel) in &mut points {
            *pos += *vel;

            xp.0 = i32::min(pos.x, xp.0);
            xp.1 = i32::max(pos.x, xp.1);
            yp.0 = i32::min(pos.y, yp.0);
            yp.1 = i32::max(pos.y, yp.1);
        }

        if yp.1 - yp.0 != 9 {
            continue;
        }

        let mut by_pos = HashSet::new();

        for &(ref pos, _) in &points {
            by_pos.insert(*pos);
        }

        for y in yp.0..=yp.1 {
            for x in xp.0..=xp.1 {
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
