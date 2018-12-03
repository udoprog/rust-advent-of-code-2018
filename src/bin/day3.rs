use aoc2018::*;
use std::error;
use std::str;

#[derive(Debug)]
struct Pair<T>(pub T, pub T);

impl<T> str::FromStr for Pair<T>
where
    T: str::FromStr,
    T::Err: 'static + Send + Sync + error::Error,
{
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut it = s.split(|c| !char::is_numeric(c)).filter(|s| s.trim() != "");
        let x = it.next().ok_or_else(|| format_err!("expected x"))?;
        let y = it.next().ok_or_else(|| format_err!("expected y"))?;
        let x = str::FromStr::from_str(x)?;
        let y = str::FromStr::from_str(y)?;
        Ok(Pair(x, y))
    }
}

fn main() -> Result<(), Error> {
    let mut duplicates = 0;
    // map out who owns each claim.
    let mut claimed_by = HashMap::<_, Vec<&str>>::new();
    let mut nonoverlapping = HashSet::new();

    // only read once so we can use references into it to avoid copying the string.
    let lines = lines!(input!("day3.txt"), String, String, Pair<u32>, Pair<u32>)
        .collect::<Result<Vec<_>, Error>>()?;

    for line in &lines {
        let (ref id, _, ref b, ref c) = *line;

        nonoverlapping.insert(id.as_str());

        for x in b.0..b.0.saturating_add(c.0) {
            for y in b.1..b.1.saturating_add(c.1) {
                let m = claimed_by.entry((x, y)).or_default();
                m.push(id.as_str());

                if m.len() == 2 {
                    duplicates += 1;
                }

                if m.len() > 1 {
                    for remove in m {
                        nonoverlapping.remove(remove);
                    }
                }
            }
        }
    }

    let nonoverlapping = nonoverlapping.into_iter().collect::<Vec<_>>();

    assert_eq!(duplicates, 104712);
    assert_eq!(nonoverlapping, vec!["#840"]);
    Ok(())
}
