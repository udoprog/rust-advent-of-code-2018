use aoc2018::*;

use std::fmt;
use std::ptr;

fn unsafe_game(players: u32, highest: u32) -> Option<u32> {
    let mut cur = Node::new(0);
    let mut scores = HashMap::<u32, u32>::new();

    for (p, marble) in (0..players).cycle().zip(1..).take(highest as usize) {
        if marble % 23 == 0 {
            cur = cur.back(7);
            let (next, last_marble) = cur.unlink();
            *scores.entry(p).or_default() += marble + last_marble;
            cur = next.expect("no more nodes");
        } else {
            cur = cur.forward(1).insert(marble);
        }
    }

    return scores.iter().max_by(|a, b| a.1.cmp(&b.1)).map(|e| *e.1);
}

fn game(players: u32, highest: u32) -> Option<u32> {
    let mut circle = VecDeque::new();
    circle.push_back(0);

    let mut cur = 0;
    let mut scores = HashMap::<u32, u32>::new();

    let it = (0..players).cycle().zip(1..).take(highest as usize);

    for (p, marble) in it {
        if marble % 23 == 0 {
            let mut score = marble;

            if cur < 7 {
                cur = circle.len() - (7 - cur);
            } else {
                cur = cur - 7;
            }

            let last_marble = circle.remove(cur).unwrap_or_default();
            score += last_marble;

            let e = scores.entry(p).or_default();

            *e += score;
        } else {
            cur += 2;

            if cur > circle.len() {
                cur = cur - circle.len();
            }

            circle.insert(cur, marble);
        }
    }

    scores.iter().max_by(|a, b| a.1.cmp(&b.1)).map(|e| *e.1)
}

fn main() -> Result<(), Error> {
    let mut it = input_str!("day9.txt").split(" ");
    let players: u32 = str::parse(it.nth(0).expect("number of players"))?;
    let highest_score: u32 = str::parse(it.nth(5).expect("points"))?;

    assert_eq!(game(10, 1618), Some(8317));
    assert_eq!(unsafe_game(10, 1618), Some(8317));

    assert_eq!(game(13, 7999), Some(146373));
    assert_eq!(unsafe_game(13, 7999), Some(146373));

    assert_eq!(game(17, 1104), Some(2764));
    assert_eq!(unsafe_game(17, 1104), Some(2764));

    assert_eq!(game(21, 6111), Some(54718));
    assert_eq!(unsafe_game(21, 6111), Some(54718));

    assert_eq!(game(30, 5807), Some(37305));
    assert_eq!(unsafe_game(30, 5807), Some(37305));

    // Part 1.
    assert_eq!(game(players, highest_score), Some(439341));
    assert_eq!(unsafe_game(players, highest_score), Some(439341));
    // Part 2.
    // Too slow:
    // assert_eq!(game(players, highest_score * 100), Some(3566801385));
    assert_eq!(unsafe_game(players, highest_score * 100), Some(3566801385));
    Ok(())
}

// Note: following is a _very_ unsafe implementation of a linked list. But it was
// the only way I could get this fast enough.
struct Data {
    prev: *mut Data,
    next: *mut Data,
    value: u32,
}

struct Node(*mut Data);

impl Node {
    fn new(value: u32) -> Node {
        let n = Box::into_raw(Box::new(Data {
            next: ptr::null_mut(),
            prev: ptr::null_mut(),
            value,
        }));

        unsafe {
            (*n).next = n;
            (*n).prev = n;
        }

        Node(n)
    }

    /// Rotate node back `c` steps.
    fn back(mut self, c: usize) -> Node {
        unsafe {
            let mut data = self.0;

            for _ in 0..c {
                data = (*data).prev;
            }

            self.0 = data;
        }

        self
    }

    /// Rotate node forward `c` steps.
    fn forward(mut self, c: usize) -> Node {
        unsafe {
            let mut data = self.0;

            for _ in 0..c {
                data = (*data).next;
            }

            self.0 = data;
        }

        self
    }

    /// Unlink the current node, returning the node immediately after this node, or `None`
    /// if there is none.
    fn unlink(mut self) -> (Option<Node>, u32) {
        use std::mem;

        let ptr = mem::replace(&mut self.0, ptr::null_mut());

        unsafe {
            // NB: only one node.
            if (*ptr).next == ptr {
                let c = Box::<Data>::from_raw(ptr);
                return (None, c.value);
            }

            let mut c = Box::<Data>::from_raw(ptr);
            (*c.prev).next = c.next;
            (*c.next).prev = c.prev;
            (Some(Node(c.next)), c.value)
        }
    }

    /// Insert a node immediately after the current node, and return the inserted node.
    fn insert(mut self, value: u32) -> Node {
        unsafe {
            let data = Box::into_raw(Box::new(Data {
                next: (*self.0).next,
                prev: self.0,
                value: value,
            }));

            (*(*self.0).next).prev = data;
            (*self.0).next = data;

            self.0 = data;
        }

        self
    }
}

impl Drop for Node {
    fn drop(&mut self) {
        // NB: Node that has been explicitly unlinked.
        if self.0 == ptr::null_mut() {
            return;
        }

        unsafe {
            let s = self.0;
            let mut c = self.0;

            while (*c).next != s {
                let d = c;
                c = (*c).next;
                Box::from_raw(d);
            }

            Box::from_raw(c);
        }
    }
}

// Note: only implemented to ease debugging.
impl fmt::Debug for Node {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        unsafe {
            let s = self.0;
            let mut c = self.0;

            while (*c).next != s {
                write!(fmt, "{:?}, ", (*c).value)?;
                c = (*c).next;
            }

            write!(fmt, "{:?}", (*c).value)?;
        }

        Ok(())
    }
}
