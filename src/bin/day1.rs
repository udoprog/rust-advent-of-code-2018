use aoc2018::*;

fn part2(mods: &[i64]) -> Option<i64> {
    let mut seen = HashSet::new();
    seen.insert(0);

    mods.iter()
        .cloned()
        .cycle()
        .scan(0, |a, b| {
            *a += b;
            Some(*a)
        })
        .filter(|f| !seen.insert(*f))
        .next()
}

fn main() -> Result<(), Error> {
    let mods = columns!("day1.txt", char::is_whitespace, i64);

    assert_eq!(497, mods.iter().cloned().sum::<i64>());
    assert_eq!(Some(558), part2(&mods));
    Ok(())
}
