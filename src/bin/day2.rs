use aoc2018::*;

/// Calculate part two.
fn part2(set: &BTreeSet<Vec<char>>) -> Option<String> {
    let mut it = set.iter().peekable();

    while let (Some(a), Some(b)) = (it.next(), it.peek()) {
        let s = a
            .iter()
            .zip(*b)
            .filter(|(a, b)| a == b)
            .map(|(a, _)| *a)
            .collect::<Vec<_>>();

        // should only differ by one character.
        if s.len() == a.len() - 1 {
            return Some(s.into_iter().collect());
        }
    }

    None
}

fn main() -> Result<(), Error> {
    let mut counts = HashMap::<_, u64>::new();

    let mut set = BTreeSet::new();

    for line in lines!(input!("day2.txt"), (String)) {
        let line = line?.0;

        let chars = line.chars().collect::<Vec<_>>();
        let mut m = HashMap::new();

        for c in chars.iter().cloned() {
            *m.entry(c).or_default() += 1;
        }

        set.insert(chars);

        // Collect and de-dup all counts.
        for v in m.values().cloned().collect::<HashSet<u64>>() {
            *counts.entry(v).or_default() += 1;
        }
    }

    let checksum = [2, 3]
        .into_iter()
        .flat_map(|k| counts.get(&k))
        .fold(1, |a, b| a * b);

    assert_eq!(checksum, 7936);
    assert_eq!(part2(&set), Some(String::from("lnfqdscwjyteorambzuchrgpx")));
    Ok(())
}
