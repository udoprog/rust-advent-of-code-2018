use aoc2018::*;

#[derive(Default, Debug)]
struct Node {
    metadata: Vec<u32>,
    children: Vec<Node>,
}

impl Node {
    fn part1sum(&self) -> u32 {
        self.metadata.iter().cloned().sum::<u32>()
            + self.children.iter().map(|c| c.part1sum()).sum::<u32>()
    }

    fn part2sum(&self) -> u32 {
        if self.children.is_empty() {
            self.metadata.iter().cloned().sum::<u32>()
        } else {
            let mut r = 0;

            for m in self.metadata.iter().cloned() {
                r += self
                    .children
                    .get(m as usize - 1)
                    .map(|c| c.part2sum())
                    .unwrap_or_default();
            }

            r
        }
    }

    fn decode(it: &mut impl Iterator<Item = u32>) -> Option<Node> {
        let children = match it.next() {
            None => return None,
            Some(first) => first,
        };

        let mut node = Node::default();
        let metadata = it.next().expect("metadata");

        for _ in 0..children {
            node.children.extend(Self::decode(it));
        }

        for _ in 0..metadata {
            node.metadata.push(it.next().expect("metadata value"));
        }

        Some(node)
    }
}

fn main() -> Result<(), Error> {
    let input = columns!(input!("day8.txt"), char::is_whitespace, u32);
    let mut it = input.iter().cloned();

    let node = Node::decode(&mut it).expect("no nodes in input");

    assert_eq!(node.part1sum(), 47647);
    assert_eq!(node.part2sum(), 23636);
    Ok(())
}
