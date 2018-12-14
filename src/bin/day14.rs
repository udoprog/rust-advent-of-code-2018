use aoc2018::*;

struct Recipe {
    pub data: Vec<usize>,
    a: usize,
    b: usize,
}

impl Recipe {
    fn new() -> Self {
        let mut data = Vec::new();
        data.push(3);
        data.push(7);

        Recipe {
            data,
            a: 0,
            b: 1,
        }
    }

    fn make(&mut self) -> usize {
        let a = self.a;
        let b = self.b;

        let mut sum = self.data[a] + self.data[b];

        let mut c = 1;

        while sum >= 10 {
            c += 1;
            self.data.push(sum % 10);
            sum /= 10;
        }

        self.data.push(sum);

        let s = self.data.len() - c;
        (&mut self.data[s..]).reverse();

        self.a = (a + self.data[a] + 1) % self.data.len();
        self.b = (b + self.data[b] + 1) % self.data.len();
        self.data.len()
    }
}

fn part1(input: usize) -> String {
    let mut recipe = Recipe::new();

    while recipe.make() < (input + 10) {
    }

    recipe.data[input..(input + 10)].iter().cloned().map(|d| d.to_string()).collect()
}

fn part2(mut input: usize) -> usize {
    let mut recipe = Recipe::new();

    let needle = {
        let mut needle = Vec::new();

        while input > 9 {
            needle.push(input % 10);
            input /= 10;
        }

        needle.push(input);

        (&mut needle[..]).reverse();
        needle
    };

    let mut ptr = 0;

    loop {
        let cur = recipe.make();

        while ptr + needle.len() < cur {
            if needle == &recipe.data[ptr..(ptr + needle.len())] {
                return ptr;
            }

            ptr += 1;
        }
    }
}

fn main() -> Result<(), Error> {
    let input = 209231;
    assert_eq!(part1(input), "6126491027");
    assert_eq!(part2(input), 20191616);
    Ok(())
}
