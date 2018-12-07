use aoc2018::*;

fn part1(deps: &HashMap<char, Vec<char>>) -> String {
    let mut left = deps.keys().cloned().collect::<BTreeSet<_>>();
    let mut satisfied = HashSet::new();
    let mut out = Vec::new();

    while !left.is_empty() {
        for c in left.iter().cloned() {
            let ok = match deps.get(&c) {
                Some(deps) => deps.iter().all(|dep| satisfied.contains(dep)),
                None => true,
            };

            if ok {
                out.push(c);
                satisfied.insert(c);
                left.remove(&c);
                break;
            }
        }
    }

    out.into_iter().collect::<String>()
}

fn part2(deps: &HashMap<char, Vec<char>>, base: u32, worker_count: usize) -> u32 {
    let mut left = deps.keys().cloned().collect::<BTreeSet<_>>();
    let mut satisfied = HashSet::new();
    let mut out = Vec::new();

    let mut workers = std::iter::repeat(())
        .map(|_| Worker {
            work: 0u32,
            current: None,
        })
        .take(worker_count)
        .collect::<Vec<_>>();

    let mut tick = 0;

    loop {
        tick += 1;

        let mut idle = Vec::new();

        for worker in &mut workers {
            worker.tick();

            if worker.work == 0 {
                if let Some(c) = worker.current.take() {
                    out.push(c);
                    satisfied.insert(c);
                }

                idle.push(worker);
            }
        }

        if left.is_empty() && idle.len() == worker_count {
            break;
        }

        // test if all dependencies are satisfied for the given node.
        let test = |c: &char| match deps.get(c) {
            Some(deps) => deps.iter().all(|dep| satisfied.contains(&dep)),
            None => true,
        };

        for (w, c) in idle
            .into_iter()
            .zip(left.iter().cloned().filter(test))
            .collect::<Vec<_>>()
        {
            w.work = base + (c as u32) - ('A' as u32) + 1;
            w.current = Some(c);
            left.remove(&c);
        }
    }

    return tick - 1;

    #[derive(Debug)]
    struct Worker {
        /// The amount of work left to do.
        work: u32,
        /// The current work.
        current: Option<char>,
    }

    impl Worker {
        fn tick(&mut self) {
            self.work = self.work.saturating_sub(1);
        }
    }
}

fn main() -> Result<(), Error> {
    let lines = input!("day7.txt").lines().collect::<Result<Vec<_>, _>>()?;
    let deps = deps(&lines);

    assert_eq!(part1(&deps), "BGJCNLQUYIFMOEZTADKSPVXRHW");
    assert_eq!(part2(&deps, 60, 5), 1017);
    Ok(())
}

fn deps(lines: &[String]) -> HashMap<char, Vec<char>> {
    let mut deps = HashMap::<char, Vec<char>>::new();

    for line in lines {
        let mut it = line.as_str().trim().split(" ");
        let before = it
            .nth(1)
            .expect("before")
            .chars()
            .next()
            .expect("before not char");
        let after = it
            .nth(5)
            .expect("after")
            .chars()
            .next()
            .expect("after not char");

        deps.entry(after).or_default().push(before);
        deps.entry(before).or_default();
    }

    deps
}
