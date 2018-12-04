use aoc2018::*;

fn main() -> Result<(), Error> {
    let mut records = Vec::new();

    for line in BufReader::new(input!("day4.txt")).lines() {
        let line = line?;
        let (date, rest) = line.split_at(18);
        let date = chrono::NaiveDateTime::parse_from_str(date, "[%Y-%m-%d %H:%M]")?;
        records.push((date, rest.trim().to_string()));
    }

    records.sort_by(|a, b| a.0.cmp(&b.0));

    let mut current = 0u32;
    let mut asleep = None;

    let mut minutes_asleep = HashMap::<_, u32>::new();
    let mut guard_asleep_at = HashMap::<_, HashMap<u32, u32>>::new();
    let mut minute_asleep = HashMap::<_, HashMap<u32, u32>>::new();

    for (date, rest) in records {
        if rest.ends_with("begins shift") {
            let guard_id = rest
                .split(" ")
                .nth(1)
                .ok_or_else(|| format_err!("no guard id"))?
                .trim_matches('#');

            current = str::parse(guard_id)?;
            asleep = None;
            continue;
        }

        match rest.as_str() {
            "falls asleep" => {
                asleep = Some(date);
            }
            "wakes up" => {
                let asleep = asleep.as_ref().unwrap();
                let count = date.signed_duration_since(asleep.clone()).num_minutes() as u32;

                *minutes_asleep.entry(current).or_default() += count;

                let mut m = asleep.minute();

                for _ in 0..count {
                    *guard_asleep_at
                        .entry(current)
                        .or_default()
                        .entry(m)
                        .or_default() += 1;

                    *minute_asleep
                        .entry(m)
                        .or_default()
                        .entry(current)
                        .or_default() += 1;

                    m += 1;
                }
            }
            other => {
                panic!("other: {}", other);
            }
        }
    }

    let max = minutes_asleep
        .into_iter()
        .max_by(|a, b| a.1.cmp(&b.1))
        .ok_or_else(|| format_err!("no solution found"))?;

    let pair = guard_asleep_at
        .remove(&max.0)
        .unwrap_or_default()
        .into_iter()
        .max_by(|a, b| a.1.cmp(&b.1))
        .ok_or_else(|| format_err!("no solution found"))?;

    let mut sleep_min = None;

    for (ts, guards) in minute_asleep {
        if let Some((guard, times)) = guards.into_iter().max_by(|a, b| a.1.cmp(&b.1)) {
            sleep_min = match sleep_min {
                Some((ts, guard, max)) if max > times => Some((ts, guard, max)),
                Some(_) | None => Some((ts, guard, times)),
            };
        }
    }

    let sleep_min = sleep_min.expect("no result found");

    assert_eq!(pair.0 * max.0, 19830);
    assert_eq!(sleep_min.0 * sleep_min.1, 43695);
    Ok(())
}
