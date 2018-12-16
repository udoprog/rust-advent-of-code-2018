use aoc2018::*;

use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Pos(i64, i64);
type Order = (i64, i64);
pub type UnitId = usize;

impl Pos {
    /// Get the order of the given position.
    pub fn order(self) -> Order {
        let Pos(x, y) = self;
        (y, x)
    }

    /// Get a collection of neighbours based on the current position.
    pub fn neighs(self) -> impl Iterator<Item = Pos> {
        let Pos(x, y) = self;
        vec![Pos(x, y - 1), Pos(x - 1, y), Pos(x + 1, y), Pos(x, y + 1)].into_iter()
    }

    /// The difference between this compared to another position.
    pub fn delta(self, other: Pos) -> (i64, i64) {
        let Pos(x0, y0) = self;
        let Pos(x1, y1) = other;
        ((x1 - x0).abs(), (y1 - y0).abs())
    }

    /// The distance between two points.
    pub fn distance(self, other: Pos) -> i64 {
        let (x, y) = self.delta(other);
        x + y
    }
}

#[derive(Debug, Clone, Default)]
pub struct State {
    position_by_unit: HashMap<UnitId, Pos>,
    unit_by_position: HashMap<Pos, UnitId>,
    walls: HashSet<Pos>,
    units: HashMap<UnitId, Unit>,
    width: i64,
    height: i64,
    debug: bool,
    sleep: u64,
    title: Option<String>,
    killed: BTreeMap<Kind, u64>,
}

impl State {
    /// Load state from the given string.
    pub fn load(input: &str) -> Result<State, Error> {
        let mut state = Self::default();
        state.sleep = 50;

        for (y, line) in input.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                let pos = Pos(x as i64, y as i64);

                state.width = i64::max(state.width, pos.0);
                state.height = i64::max(state.height, pos.1);

                let kind = match c {
                    '#' => {
                        state.walls.insert(pos);
                        continue;
                    }
                    'G' => Kind::Goblin,
                    'E' => Kind::Elf,
                    '.' => continue,
                    c => bail!("unsupported: {}", c),
                };

                let unit = Unit::new(kind);
                let id = state.units.len();
                state.units.insert(id, unit);
                state.position_by_unit.insert(id, pos);
                state.unit_by_position.insert(pos, id);
            }
        }

        Ok(state)
    }

    /// Enable more verbose debugging.
    pub fn debug(mut self) -> Self {
        self.debug = true;
        self
    }

    /// Remove a unit by ID.
    pub fn remove_unit(&mut self, id: UnitId) -> Result<(), Error> {
        if self.units.remove(&id).is_none() {
            bail!("no such unit: {}", id);
        }

        let pos = match self.position_by_unit.remove(&id) {
            Some(pos) => pos,
            None => bail!("no position for unit: {}", id),
        };

        let old_id = match self.unit_by_position.remove(&pos) {
            Some(id) => id,
            None => bail!("no unit for position: {:?}", pos),
        };

        if old_id != id {
            bail!("wrong unit for position: {:?}", pos);
        }

        Ok(())
    }

    /// Moves a unit from one position to another.
    pub fn move_unit(&mut self, from: Pos, to: Pos) -> Result<(), Error> {
        let id = match self.unit_by_position.remove(&from) {
            Some(id) => id,
            None => bail!("no unit at position: {:?}", from),
        };

        if let Some(other_id) = self.unit_by_position.insert(to, id) {
            bail!(
                "there was already a unit at position {:?}: {}",
                to,
                other_id
            );
        }

        if let Some(old_pos) = self.position_by_unit.insert(id, to) {
            if old_pos != from {
                bail!(
                    "wrong position `{:?}` recorded for unit `{}`, expected: {:?}",
                    old_pos,
                    id,
                    from
                );
            }
        }

        Ok(())
    }

    /// Find the position of a unit.
    fn find_unit_position(&self, id: UnitId) -> Result<Pos, Error> {
        match self.position_by_unit.get(&id) {
            Some(pos) => Ok(*pos),
            None => bail!("no position for unit: {}", id),
        }
    }

    /// Find all units to perform actions based on their order.
    pub fn find_priority_units(&self) -> Vec<UnitId> {
        let mut units = self
            .position_by_unit
            .iter()
            .map(|(id, p)| (*id, *p))
            .collect::<Vec<_>>();

        units.sort_by_key(|(_, p)| p.order());
        units.into_iter().map(|(id, _)| id).collect::<Vec<_>>()
    }

    /// Find the given unit by its position.
    pub fn find_unit_by_position(&self, pos: Pos) -> Option<&Unit> {
        let id = match self.unit_by_position.get(&pos).cloned() {
            Some(id) => id,
            None => return None,
        };

        self.units.get(&id)
    }

    /// Get unit data.
    pub fn find_unit(&self, id: UnitId) -> Result<&Unit, Error> {
        match self.units.get(&id) {
            None => bail!("no unit by id: {}", id),
            Some(unit) => Ok(unit),
        }
    }

    pub fn find_mut_unit(&mut self, id: UnitId) -> Result<&mut Unit, Error> {
        match self.units.get_mut(&id) {
            None => bail!("no unit by id: {}", id),
            Some(unit) => Ok(unit),
        }
    }

    /// Find the target that is closest, or `None` if none if they are not reachable.
    fn find_next_step(&self, from: Pos, targets: impl IntoIterator<Item = Pos>) -> Option<Pos> {
        use self::hash_map::Entry;

        let targets = targets.into_iter().collect::<HashSet<_>>();

        let found = {
            let mut queue = VecDeque::new();
            queue.push_back((from, 0));

            let mut visited = HashSet::new();
            let mut found = None;

            while let Some((p, d)) = queue.pop_front() {
                if !visited.insert(p) {
                    continue;
                }

                if targets.contains(&p) {
                    found = Some((p, d));
                    break;
                }

                if p != from {
                    if self.is_unit_at(p) || self.is_wall(p) {
                        continue;
                    }
                }

                queue.extend(p.neighs().map(|n| (n, d + 1)));
            }

            found
        };

        let (target, target_distance) = match found {
            Some(found) => found,
            None => return None,
        };

        let mut dist = HashMap::new();

        let mut queue = VecDeque::new();
        queue.push_back((target, 0));

        while let Some((p, d)) = queue.pop_front() {
            if p != target {
                if self.is_unit_at(p) || self.is_wall(p) {
                    continue;
                }
            }

            match dist.entry(p) {
                Entry::Vacant(e) => {
                    e.insert(d);
                }
                Entry::Occupied(mut e) => {
                    if *e.get() <= d {
                        continue;
                    }

                    e.insert(d);
                }
            }

            queue.extend(p.neighs().map(|n| (n, d + 1)));
        }

        let mut candidates = Vec::new();

        for n in from.neighs() {
            if let Some(d) = dist.get(&n).cloned() {
                if d == target_distance - 1 {
                    candidates.push(n);
                }
            }
        }

        candidates.sort_by_key(|c| c.order());
        candidates.into_iter().next()
    }

    /// Find targets to attack.
    fn find_attack_target(&self, my_id: UnitId) -> Result<Option<UnitId>, Error> {
        let me = self.find_unit(my_id)?;
        let my_pos = self.find_unit_position(my_id)?;

        let mut attack = None;

        for target_id in self.find_priority_units() {
            let target_unit = self.find_unit(target_id)?;

            if !me.is_target(target_unit) {
                continue;
            }

            let other_pos = self.find_unit_position(target_id)?;

            if my_pos.distance(other_pos) == 1 {
                attack = match attack {
                    Some((_, hit_points)) if hit_points > target_unit.hit_points => {
                        Some((target_id, target_unit.hit_points))
                    }
                    None => Some((target_id, target_unit.hit_points)),
                    other => other,
                };
            }
        }

        Ok(attack.map(|(id, _)| id))
    }

    /// Is the given grid position empty.
    fn is_wall(&self, pos: Pos) -> bool {
        self.walls.get(&pos).is_some()
    }

    /// Test if the given position has a unit.
    pub fn is_unit_at(&self, pos: Pos) -> bool {
        self.unit_by_position.contains_key(&pos)
    }

    /// Test if the given id is a unit.
    pub fn is_unit(&self, id: UnitId) -> bool {
        self.units.contains_key(&id)
    }

    pub fn simulate(&mut self) -> Result<u64, Error> {
        use std::io::{self, Write};

        let stdout = io::stdout();
        let mut out = stdout.lock();

        // execute turns
        for tick in 0u64.. {
            if self.debug {
                writeln!(out, "[ENTER] to progress...")?;
                let mut s = String::new();
                std::io::stdin().read_line(&mut s)?;
            } else {
                if self.sleep > 0 {
                    std::thread::sleep(std::time::Duration::from_millis(self.sleep));
                }

                write!(out, "{}[2J", 27 as char)?;
            }

            writeln!(out, "{}", Display(&self))?;

            if let Some(title) = self.title.as_ref() {
                writeln!(out, "{}", title)?;
            }

            writeln!(out, "Killed: {:?}", self.killed)?;
            writeln!(out, "Tick: {}", tick)?;

            let prioritized_units = self.find_priority_units();

            for my_id in prioritized_units.iter().cloned() {
                if !self.is_unit(my_id) {
                    continue;
                }

                let me = self.find_unit(my_id)?;

                // units to attack.
                let mut attack = self.find_attack_target(my_id)?;

                // if there is nothing to attack, then move.
                if attack.is_none() {
                    // units to target.
                    let mut targets = Vec::new();

                    for target_id in prioritized_units.iter().cloned().filter(|id| my_id != *id) {
                        if !self.is_unit(target_id) {
                            continue;
                        }

                        let other_pos = self.find_unit_position(target_id)?;
                        let target = self.find_unit(target_id)?;

                        if !me.is_target(target) {
                            continue;
                        }

                        targets.push(other_pos);
                    }

                    let my_pos = self.find_unit_position(my_id)?;

                    if let Some(next_pos) = self.find_next_step(my_pos, targets) {
                        self.move_unit(my_pos, next_pos)?;
                        attack = self.find_attack_target(my_id)?;
                    }
                }

                if let Some(enemy_id) = attack {
                    let attack_power = self.find_unit(my_id)?.attack_power;

                    let enemy = self.find_mut_unit(enemy_id)?;
                    enemy.hit_points = enemy.hit_points.saturating_sub(attack_power);

                    if enemy.hit_points == 0 {
                        let killed_kind = enemy.kind;
                        self.remove_unit(enemy_id)?;
                        *self.killed.entry(killed_kind).or_default() += 1;
                    }

                    continue;
                }
            }

            let mut kinds = HashSet::new();
            let mut hit_points = 0;

            for my_id in prioritized_units.iter().cloned() {
                if !self.is_unit(my_id) {
                    continue;
                }

                let unit = self.find_unit(my_id)?;

                if self.debug {
                    println!("{:?}", unit);
                }

                kinds.insert(unit.kind);
                hit_points += unit.hit_points;
            }

            if kinds.len() == 1 {
                return Ok(tick * hit_points);
            }
        }

        bail!("could not find a result");
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Kind {
    Goblin,
    Elf,
}

#[derive(Debug, Clone)]
pub struct Unit {
    kind: Kind,
    hit_points: u64,
    attack_power: u64,
}

impl Unit {
    pub fn new(kind: Kind) -> Self {
        Self {
            kind,
            hit_points: 200,
            attack_power: 3,
        }
    }

    /// Test if other unit is a valid target.
    pub fn is_target(&self, other: &Unit) -> bool {
        match (self.kind, other.kind) {
            (Kind::Elf, Kind::Elf) => false,
            (Kind::Goblin, Kind::Goblin) => false,
            (_, _) => true,
        }
    }
}

fn save_the_elves(mut original: State) -> Result<u64, Error> {
    original.sleep = 20;

    for ap in 4.. {
        let mut state = original.clone();
        state.title = Some(format!("Attack Power: {}", ap));

        for u in state.units.values_mut() {
            if let Kind::Elf = u.kind {
                u.attack_power = ap;
            }
        }

        let result = state.simulate()?;

        if state.killed.get(&Kind::Elf).cloned() == None {
            return Ok(result);
        }
    }

    bail!("no result :(");
}

fn main() -> Result<(), Error> {
    assert_eq!(State::load(input_str!("day15a.txt"))?.simulate()?, 36334);
    assert_eq!(State::load(input_str!("day15b.txt"))?.simulate()?, 39514);
    assert_eq!(State::load(input_str!("day15c.txt"))?.simulate()?, 27755);
    assert_eq!(State::load(input_str!("day15d.txt"))?.simulate()?, 28944);
    assert_eq!(State::load(input_str!("day15e.txt"))?.simulate()?, 18740);
    assert_eq!(State::load(input_str!("day15.txt"))?.simulate()?, 207059);
    assert_eq!(
        save_the_elves(State::load(input_str!("day15.txt"))?)?,
        49120
    );
    Ok(())
}

pub struct Display<'a>(&'a State);

impl fmt::Display for Display<'_> {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Display(state) = *self;

        for y in 0..=state.height {
            for x in 0..=state.width {
                let p = Pos(x, y);

                match state.walls.contains(&p) {
                    true => "🧱".fmt(fmt)?,
                    false => match state.find_unit_by_position(p) {
                        Some(unit) => match unit.kind {
                            Kind::Goblin => "👹".fmt(fmt)?,
                            Kind::Elf => "🧝".fmt(fmt)?,
                        },
                        None => "⬛".fmt(fmt)?,
                    },
                }
            }

            writeln!(fmt, "")?;
        }

        Ok(())
    }
}
