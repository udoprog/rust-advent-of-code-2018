use aoc2018::*;

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Device([u64; 4]);

impl Device {
    /// Try to decode a device.
    /// Devices take the form `[a, b, c, d]` representing all registries.
    pub fn decode(state: &str) -> Option<Device> {
        let mut it = state
            .trim_matches(|c| c == '[' || c == ']')
            .split(", ")
            .flat_map(|d| str::parse(d).ok());

        Some(Device([it.next()?, it.next()?, it.next()?, it.next()?]))
    }

    pub fn reg(&mut self, reg: u64) -> Result<&mut u64, Error> {
        match self.0.get_mut(reg as usize) {
            Some(reg) => Ok(reg),
            None => bail!("no such register: {}", reg),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum OpCode {
    Addr,
    Addi,
    Mulr,
    Muli,
    Banr,
    Bani,
    Borr,
    Bori,
    Setr,
    Seti,
    Gtir,
    Gtri,
    Gtrr,
    Eqir,
    Eqri,
    Eqrr,
}

impl OpCode {
    /// Iterate over all variants.
    fn variants() -> impl Iterator<Item = OpCode> {
        use self::OpCode::*;

        vec![
            Addr, Addi, Mulr, Muli, Banr, Bani, Borr, Bori, Setr, Seti, Gtir, Gtri, Gtrr, Eqir,
            Eqri, Eqrr,
        ]
        .into_iter()
    }

    fn apply(&self, d: &mut Device, inputs: &[u64; 2], o: u64) -> Result<(), Error> {
        use self::OpCode::*;

        let [a, b] = *inputs;

        *d.reg(o)? = match *self {
            Addr => *d.reg(a)? + *d.reg(b)?,
            Addi => *d.reg(a)? + b,
            Mulr => *d.reg(a)? * *d.reg(b)?,
            Muli => *d.reg(a)? * b,
            Banr => *d.reg(a)? & *d.reg(b)?,
            Bani => *d.reg(a)? & b,
            Borr => *d.reg(a)? | *d.reg(b)?,
            Bori => *d.reg(a)? | b,
            Setr => *d.reg(a)?,
            Seti => a,
            Gtir => {
                if a > *d.reg(b)? {
                    1
                } else {
                    0
                }
            }
            Gtri => {
                if *d.reg(a)? > b {
                    1
                } else {
                    0
                }
            }
            Gtrr => {
                if *d.reg(a)? > *d.reg(b)? {
                    1
                } else {
                    0
                }
            }
            Eqir => {
                if a == *d.reg(b)? {
                    1
                } else {
                    0
                }
            }
            Eqri => {
                if *d.reg(a)? == b {
                    1
                } else {
                    0
                }
            }
            Eqrr => {
                if *d.reg(a)? == *d.reg(b)? {
                    1
                } else {
                    0
                }
            }
        };

        Ok(())
    }
}

/// An instruction.
#[derive(Debug)]
pub struct Instruction {
    op_code: u64,
    inputs: [u64; 2],
    output: u64,
}

impl Instruction {
    pub fn decode(state: &str) -> Option<Instruction> {
        let mut it = state.split(" ").flat_map(|d| str::parse(d).ok());

        Some(Instruction {
            op_code: it.next()?,
            inputs: [it.next()?, it.next()?],
            output: it.next()?,
        })
    }
}

#[derive(Debug, Default)]
struct Registry(HashMap<u64, HashSet<OpCode>>);

impl Registry {
    /// Try to reduce the number of definitive matches.
    pub fn regress(&mut self) -> Vec<(u64, OpCode)> {
        let mut known = Vec::new();
        let mut current = 0;

        self.known(&mut known);

        while current != known.len() {
            current = known.len();

            for (known_num, known_op) in known.iter().cloned() {
                for (num, values) in self.0.iter_mut() {
                    if *num == known_num {
                        values.clear();
                        values.insert(known_op);
                        continue;
                    }

                    values.remove(&known_op);
                }
            }

            known.clear();
            self.known(&mut known);
        }

        known
    }

    /// Extract exactly known op-codes.
    fn known(&self, known: &mut Vec<(u64, OpCode)>) {
        for (key, value) in &self.0 {
            if value.len() == 1 {
                if let Some(op) = value.iter().cloned().next() {
                    known.push((*key, op));
                }
            }
        }
    }
}

struct Decoder {
    codes: HashMap<u64, OpCode>,
}

impl Decoder {
    pub fn new(codes: impl IntoIterator<Item = (u64, OpCode)>) -> Decoder {
        Decoder {
            codes: codes.into_iter().collect(),
        }
    }

    pub fn decode(&self, code: u64) -> Result<OpCode, Error> {
        match self.codes.get(&code).cloned() {
            Some(op) => Ok(op),
            None => bail!("no such op: {}", code),
        }
    }
}

fn main() -> Result<(), Error> {
    let mut it = input_str!("day16.txt").lines();

    let mut part1 = 0;

    let mut registry = Registry::default();

    // NB: all op codes are initially possible for all op numbers.
    for c in 0..16u64 {
        for op in OpCode::variants() {
            registry.0.entry(c).or_default().insert(op);
        }
    }

    while let Some(test) = Test::decode(&mut it) {
        let mut matches = HashSet::new();

        for op in OpCode::variants() {
            let mut device = test.before.clone();
            op.apply(&mut device, &test.inst.inputs, test.inst.output)?;

            if device == test.after {
                matches.insert(op);
            } else {
                if let Some(codes) = registry.0.get_mut(&test.inst.op_code) {
                    codes.remove(&op);
                }
            }
        }

        // definitive proof that a specific op-code is the one.
        if matches.len() == 1 {
            if let Some(op) = matches.iter().cloned().next() {
                if let Some(values) = registry.0.get_mut(&test.inst.op_code) {
                    values.clear();
                    values.insert(op);
                }
            }
        }

        if matches.len() >= 3 {
            part1 += 1;
        }
    }

    let known = registry.regress();

    assert_eq!(known.len(), 16);
    assert_eq!(part1, 596);

    let decoder = Decoder::new(known);

    assert_eq!(it.next(), Some(""));

    let mut device = Device::default();

    for inst in it.flat_map(Instruction::decode) {
        let op = decoder.decode(inst.op_code)?;
        op.apply(&mut device, &inst.inputs, inst.output)?;
    }

    assert_eq!(*device.reg(0)?, 554);
    Ok(())
}

#[derive(Debug)]
struct Test {
    before: Device,
    inst: Instruction,
    after: Device,
}

impl Test {
    fn decode<'a>(it: &mut impl Iterator<Item = &'a str>) -> Option<Test> {
        let before = it.next()?;

        if before == "" {
            return None;
        }

        let inst = it.next()?;
        let after = it.next()?;
        let blank = it.next()?;

        if !before.starts_with("Before: ") {
            return None;
        }

        if !after.starts_with("After: ") {
            return None;
        }

        if blank != "" {
            return None;
        }

        let before = Device::decode(before.split(": ").nth(1)?.trim())?;
        let inst = Instruction::decode(&inst)?;
        let after = Device::decode(after.split(": ").nth(1)?.trim())?;

        Some(Test {
            before,
            inst,
            after,
        })
    }
}
