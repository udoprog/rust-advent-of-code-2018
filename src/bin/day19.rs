use aoc2018::*;
use std::fmt;

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Device {
    registers: [i64; 6],
    /// Which register contains the current instruction.
    ip: usize,
}

impl Device {
    pub fn ip(&mut self) -> Result<&mut i64, Error> {
        match self.registers.get_mut(self.ip) {
            Some(reg) => Ok(reg),
            None => bail!("no ip register: {}", self.ip),
        }
    }

    pub fn reg(&mut self, reg: i64) -> Result<&mut i64, Error> {
        match self.registers.get_mut(reg as usize) {
            Some(reg) => Ok(reg),
            None => bail!("no such register: {}", reg),
        }
    }

    fn reg_name(&self, reg: i64) -> String {
        if reg as usize == self.ip {
            String::from("%ip")
        } else {
            format!("%{}", ['a', 'b', 'c', 'd', 'e', 'f'][reg as usize])
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

impl fmt::Display for OpCode {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        use self::OpCode::*;

        let name = match *self {
            Addr => "addr",
            Addi => "addi",
            Mulr => "mulr",
            Muli => "muli",
            Banr => "banr",
            Bani => "bani",
            Borr => "borr",
            Bori => "bori",
            Setr => "setr",
            Seti => "seti",
            Gtir => "gtir",
            Gtri => "gtri",
            Gtrr => "gtrr",
            Eqir => "eqir",
            Eqri => "eqri",
            Eqrr => "eqrr",
        };

        name.fmt(fmt)
    }
}

impl OpCode {
    pub fn names(&self, d: &Device, inputs: [i64; 2]) -> [String; 2] {
        use self::OpCode::*;

        let [a, b] = inputs;

        match *self {
            Addr => [d.reg_name(a), d.reg_name(b)],
            Addi => [d.reg_name(a), b.to_string()],
            Mulr => [d.reg_name(a), d.reg_name(b)],
            Muli => [d.reg_name(a), b.to_string()],
            Banr => [d.reg_name(a), d.reg_name(b)],
            Bani => [d.reg_name(a), b.to_string()],
            Borr => [d.reg_name(a), d.reg_name(b)],
            Bori => [d.reg_name(a), b.to_string()],
            Setr => [d.reg_name(a), String::from("/* ignore */")],
            Seti => [a.to_string(), String::from("/* ignore */")],
            Gtir => [a.to_string(), d.reg_name(b)],
            Gtri => [d.reg_name(a), b.to_string()],
            Gtrr => [d.reg_name(a), d.reg_name(b)],
            Eqir => [a.to_string(), d.reg_name(b)],
            Eqri => [d.reg_name(a), b.to_string()],
            Eqrr => [d.reg_name(a), d.reg_name(b)],
        }
    }

    pub fn decode(input: &str) -> Option<OpCode> {
        use self::OpCode::*;

        let out = match input {
            "addr" => Addr,
            "addi" => Addi,
            "mulr" => Mulr,
            "muli" => Muli,
            "banr" => Banr,
            "bani" => Bani,
            "borr" => Borr,
            "bori" => Bori,
            "setr" => Setr,
            "seti" => Seti,
            "gtir" => Gtir,
            "gtri" => Gtri,
            "gtrr" => Gtrr,
            "eqir" => Eqir,
            "eqri" => Eqri,
            "eqrr" => Eqrr,
            _ => return None,
        };

        Some(out)
    }

    fn apply(&self, d: &mut Device, inputs: &[i64; 2], o: i64) -> Result<(), Error> {
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
#[derive(Debug, Clone)]
pub struct Instruction {
    op_code: OpCode,
    inputs: [i64; 2],
    output: i64,
}

impl Instruction {
    pub fn decode(state: &str) -> Option<Instruction> {
        let mut it = state.split(" ");
        let op_code = OpCode::decode(it.next()?)?;
        let mut it = it.flat_map(|d| str::parse(d).ok());

        Some(Instruction {
            op_code,
            inputs: [it.next()?, it.next()?],
            output: it.next()?,
        })
    }
}

/// Convert assembler into a more conveneint (named) format that is easier to disassemble.
#[allow(unused)]
fn names<'a>(mut it: impl Iterator<Item = &'a str>) -> Result<(), Error> {
    let mut device = Device::default();

    device.ip = it
        .next()
        .and_then(|s| {
            if s.starts_with("#ip") {
                str::parse(s.split(" ").nth(1)?).ok()
            } else {
                None
            }
        })
        .ok_or_else(|| format_err!("expected #ip statement"))?;

    for (idx, inst) in it.flat_map(Instruction::decode).enumerate() {
        let [a, b] = inst.op_code.names(&device, inst.inputs);
        println!(
            "{:02}: {} {} {} {}",
            idx,
            inst.op_code,
            a,
            b,
            device.reg_name(inst.output)
        );
    }

    Ok(())
}

fn solve<'a, V>(
    mut visuals: V,
    mut it: impl Iterator<Item = &'a str>,
    initial: i64,
) -> Result<i64, Error>
where
    V: Visuals,
{
    let mut device = Device::default();

    device.ip = it
        .next()
        .and_then(|s| {
            if s.starts_with("#ip") {
                str::parse(s.split(" ").nth(1)?).ok()
            } else {
                None
            }
        })
        .ok_or_else(|| format_err!("expected #ip statement"))?;

    *device.reg(0)? = initial;

    V::setup();

    let instructions = it.flat_map(Instruction::decode).collect::<Vec<_>>();

    let mut prev = None;

    loop {
        let ip = *device.ip()?;

        let inst = match instructions.get(ip as usize) {
            Some(inst) => inst,
            None => break,
        };

        prev = Some(device.clone());
        inst.op_code.apply(&mut device, &inst.inputs, inst.output)?;

        visuals.observe(inst.clone());
        visuals.draw(&device, prev.as_ref());

        *device.ip()? += 1;
    }

    V::done(&mut device)?;

    Ok(*device.reg(0)?)
}

fn part2() -> u64 {
    // NB: extracted from input by running it in interactive mode for a while :)
    let d = 10551330;

    let mut res = 0u64;

    for f in 1..=d {
        for b in 1..=d {
            let c = f * b;

            if c == d {
                res += f;
            }

            if c > d {
                break;
            }
        }
    }

    res
}

fn main() -> Result<(), Error> {
    assert_eq!(
        solve(NoopVisuals, input_str!("day19.txt").lines(), 0)?,
        2304
    );
    assert_eq!(part2(), 28137600);

    names(input_str!("day19.txt").lines())?;

    // Note: this is the interactive visualization used to extract inputs.
    // You _will_ have to CTRL+C to exit.
    solve(
        NcursesVisuals::new(0).interactive(),
        input_str!("day19.txt").lines(),
        0,
    )?;
    Ok(())
}

trait Visuals {
    fn setup();

    fn done(device: &mut Device) -> Result<(), Error>;

    fn observe(&mut self, inst: Instruction);

    fn draw(&mut self, device: &Device, prev: Option<&Device>);
}

struct NoopVisuals;

impl Visuals for NoopVisuals {
    fn setup() {}

    fn done(_: &mut Device) -> Result<(), Error> {
        Ok(())
    }

    fn observe(&mut self, _: Instruction) {}

    fn draw(&mut self, _: &Device, _: Option<&Device>) {}
}

struct NcursesVisuals {
    sleep: u64,
    interactive: bool,
    last: Vec<Instruction>,
    changed: HashSet<usize>,
}

impl NcursesVisuals {
    pub fn new(sleep: u64) -> Self {
        Self {
            sleep,
            interactive: false,
            last: Default::default(),
            changed: Default::default(),
        }
    }

    pub fn interactive(mut self) -> Self {
        self.interactive = true;
        self
    }
}

impl Visuals for NcursesVisuals {
    fn setup() {
        ncurses::initscr();
        ncurses::noecho();
        ncurses::curs_set(ncurses::CURSOR_VISIBILITY::CURSOR_INVISIBLE);
    }

    fn done(device: &mut Device) -> Result<(), Error> {
        let a = device.reg(0)?.to_string();

        ncurses::mv(12, 2);
        ncurses::printw("Result is ");

        ncurses::attron(ncurses::A_BLINK() | ncurses::A_STANDOUT());
        ncurses::printw(&a);
        ncurses::attroff(ncurses::A_BLINK() | ncurses::A_STANDOUT());

        ncurses::printw(", press [enter] to exit...");

        loop {
            let c = ncurses::getch();

            if c == 10 {
                break;
            }
        }

        ncurses::endwin();
        Ok(())
    }

    fn observe(&mut self, inst: Instruction) {
        self.last.push(inst);
    }

    fn draw(&mut self, device: &Device, prev: Option<&Device>) {
        if let Some(prev) = prev {
            self.changed.clear();
            self.changed.extend(
                prev.registers
                    .iter()
                    .cloned()
                    .zip(device.registers.iter().cloned())
                    .enumerate()
                    .filter(|(_, (a, b))| a != b)
                    .map(|(i, _)| i),
            );
        }

        ncurses::erase();

        ncurses::attron(ncurses::A_UNDERLINE());
        ncurses::mvprintw(0, 2, "Instructions");
        ncurses::mvprintw(0, 16, "Registers");
        ncurses::attroff(ncurses::A_UNDERLINE());

        let (mut width, mut height) = (0, 0);
        ncurses::getmaxyx(ncurses::stdscr(), &mut height, &mut width);

        for (line, inst) in self.last[self.last.len().saturating_sub(10)..]
            .iter()
            .enumerate()
        {
            let [a, b] = inst.inputs;
            let c = inst.output;

            let standout = self.last.len() == (line + 1) || line == 9;

            if standout {
                ncurses::mv(line as i32 + 1, 1);
                ncurses::printw(">");
                ncurses::attron(ncurses::A_STANDOUT());
            } else {
                ncurses::mv(line as i32 + 1, 2);
            }

            ncurses::printw(&format!("{} {}, {}, {}", inst.op_code, a, b, c));

            if standout {
                ncurses::attroff(ncurses::A_STANDOUT());
            }
        }

        for (line, (name, value)) in ['0', '1', '2', '3', '4', '5']
            .into_iter()
            .zip(device.registers.iter())
            .enumerate()
        {
            let c = self.changed.contains(&line);

            ncurses::mv(line as i32 + 1, 16);
            ncurses::printw(&format!("{} = ", name));

            if c {
                ncurses::attron(ncurses::A_STANDOUT());
            }

            ncurses::printw(&value.to_string());

            if c {
                ncurses::attroff(ncurses::A_STANDOUT());
            }
        }

        if self.interactive {
            ncurses::mv(12, 2);
            ncurses::printw("press [space] to step...");
            ncurses::refresh();

            loop {
                let c = ncurses::getch();

                if c == 32 {
                    break;
                }
            }
        } else {
            if self.sleep > 0 {
                std::thread::sleep(std::time::Duration::from_millis(self.sleep));
            }
        }
    }
}
