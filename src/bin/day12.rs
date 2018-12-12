use aoc2018::*;

use std::fmt;

struct DisplaySet<'a>(&'a HashSet<i64>);

impl fmt::Display for DisplaySet<'_> {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some((min, max)) = self.0.iter().cloned().minmax().into_option() {
            for i in min..=max {
                match self.0.contains(&i) {
                    true => '#'.fmt(fmt)?,
                    false => '.'.fmt(fmt)?,
                }
            }
        }

        Ok(())
    }
}

struct Display<'a>(&'a VecDeque<bool>);

impl fmt::Display for Display<'_> {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        for b in self.0.iter().cloned() {
            match b {
                true => '#'.fmt(fmt)?,
                false => '.'.fmt(fmt)?,
            }
        }

        Ok(())
    }
}

/// Better implementation that uses sparse sets to store and query the patterns.
///
/// Has not been implemented to detect linearly shifting plants yet.
fn better(state: &[bool], m: &HashMap<Vec<bool>, bool>, generations: usize) -> i64 {
    let patterns = m
        .iter()
        .filter(|e| *e.1)
        .map(|e| {
            (-2i64..)
                .zip(e.0.iter().cloned())
                .filter(|e| e.1)
                .map(|e| e.0)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut state = (0i64..)
        .zip(state.iter().cloned())
        .filter(|e| e.1)
        .map(|e| e.0)
        .collect::<HashSet<_>>();

    for g in 0..generations {
        println!("{}", DisplaySet(&state));

        let (min, max) = state
            .iter()
            .cloned()
            .minmax()
            .into_option()
            .expect("expected min and max");

        state = (min - 3..=max + 3)
            .filter(|idx| {
                patterns.iter().any(|pat| {
                    [-2, -1, 0, 1, 2].into_iter().all(|t| {
                        if pat.contains(t) {
                            state.contains(&(*idx + *t))
                        } else {
                            !state.contains(&(*idx + *t))
                        }
                    })
                })
            })
            .collect::<HashSet<_>>();
    }

    state.into_iter().sum()
}

fn calculate(state: &[bool], m: &HashMap<Vec<bool>, bool>, generations: usize) -> i64 {
    use std::iter;

    let mut state = state.iter().cloned().collect::<VecDeque<_>>();

    let mut seen = None;

    let mut index = 0i64;

    let sum = |state: &VecDeque<bool>, index: i64| {
        state
            .iter()
            .cloned()
            .zip(index..)
            .filter(|(c, _)| *c)
            .map(|(_, i)| i)
            .sum::<i64>()
    };

    for gen in 0usize..generations {
        if let Some(m) = state.iter().take(3).position(|c| *c) {
            index -= (3 - m) as i64;

            for _ in 0..3 - m {
                state.push_front(false);
            }
        }

        if let Some(m) = state.iter().rev().take(3).position(|c| *c) {
            for _ in 0..3 - m {
                state.push_back(false);
            }
        }

        let mut next = VecDeque::new();

        for i in 0..state.len() {
            let mut palette = Vec::with_capacity(5);

            if i < 2 {
                palette.extend(iter::repeat(false).take(2 - i));
            }

            for si in i.saturating_sub(2)..usize::min(i + 3, state.len()) {
                palette.extend(state.get(si));
            }

            if i + 3 >= state.len() {
                palette.extend(iter::repeat(false).take(3 - (state.len() - i)));
            }

            if let Some(m) = m.get(&palette).cloned() {
                next.push_back(m);
            } else {
                next.push_back(false);
            }
        }

        state = next;

        // Reduce the state as much as possible.
        while let Some(false) = state.front().cloned() {
            index += 1;
            state.pop_front();
        }

        while let Some(false) = state.back().cloned() {
            state.pop_back();
        }

        let current = state.iter().cloned().collect::<Vec<_>>();

        println!("{}", Display(&state));

        if let Some((last, prev)) = seen.as_ref() {
            if last == &current {
                index += (generations - gen - 1) as i64 * (index - prev);
                return sum(&state, index);
            }
        }

        seen = Some((current, index));
    }

    sum(&state, index)
}

fn main() -> Result<(), Error> {
    //let lines = lines!(input!("day12.txt"), u32).collect::<Result<Vec<_>, _>>()?;
    //let columns = columns!(input!("day12.txt"), char::is_whitespace, u32);

    let lines = input_str!("day12.txt").lines().collect::<Vec<_>>();

    let state = lines[0]
        .split(": ")
        .nth(1)
        .expect("initial state")
        .trim()
        .chars()
        .map(|c| c == '#')
        .collect::<Vec<_>>();

    let mut m = HashMap::<Vec<bool>, bool>::new();

    for line in lines[1..].iter().cloned() {
        let line = line.trim();

        if line == "" {
            continue;
        }

        let from = line.split(" => ").nth(0).expect("from").trim();

        let to = match line.split(" => ").nth(1).expect("to").trim() {
            "." => false,
            "#" => true,
            _ => panic!("bad translation"),
        };

        m.insert(from.chars().map(|c| c == '#').collect(), to);
    }

    assert_eq!(better(&state, &m, 20), 3061);
    assert_eq!(calculate(&state, &m, 20), 3061);
    assert_eq!(calculate(&state, &m, 50000000000), 4049999998575);
    Ok(())
}
