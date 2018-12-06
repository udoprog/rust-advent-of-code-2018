use aoc2018::*;

fn is_polar(a: char, b: char) -> bool {
    let polar = b.is_uppercase() && b.to_lowercase().next() == Some(a)
        || a.is_uppercase() && a.to_lowercase().next() == Some(b);

    polar
}

/// Naive implementation. Just reduce two inputs.
fn naive(input: &str) -> usize {
    let mut chain = input.trim().chars().collect::<Vec<_>>();

    loop {
        let mut it = chain.iter().cloned().peekable();

        let mut out = Vec::new();

        while let Some(a) = it.next() {
            let b = it.peek().cloned();

            if let Some(b) = b {
                if is_polar(a, b) {
                    it.next();
                    continue;
                }
            }

            out.push(a);
        }

        if out.len() == chain.len() {
            return chain.len();
        }

        chain = out;
    }
}

/// Clever implementation.
fn clever(input: &str) -> usize {
    let mut stack = Vec::new();

    for c in input.trim().chars() {
        match stack.last().cloned() {
            Some(n) if is_polar(n, c) => {
                stack.pop();
            },
            _ => {
                stack.push(c);
            },
        }
    }

    stack.len()
}

/// Test all against the given input, after removing each unique character from the input.
fn test_with_removal(input: &str, f: impl Fn(&str) -> usize) -> Option<usize> {
    let chars = input
        .trim()
        .chars()
        .flat_map(|c| c.to_lowercase())
        .collect::<HashSet<_>>();

    let mut max = None;

    for remove in chars {
        let s = input
            .chars()
            .filter(|c| !c.eq_ignore_ascii_case(&remove))
            .collect::<String>();
        let l = f(&s);

        max = match max {
            Some(m) if m > l => Some(l),
            None => Some(l),
            Some(m) => Some(m),
        };
    }

    max
}

fn main() -> Result<(), Error> {
    // Part 1.
    assert_eq!(naive("dabAcCaCBAcCcaDA"), 10);
    assert_eq!(naive(input_str!("day5.txt")), 11364);
    assert_eq!(clever(input_str!("day5.txt")), 11364);

    // Part 2.
    assert_eq!(test_with_removal(input_str!("day5.txt"), naive), Some(4212));
    assert_eq!(test_with_removal(input_str!("day5.txt"), clever), Some(4212));
    Ok(())
}
