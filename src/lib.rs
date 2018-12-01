pub use failure::{Error, format_err, ResultExt};
pub use nalgebra as na;
pub use std::io::{BufRead, BufReader};
pub use num::{BigInt, BigUint};

/// Get the input as a string.
#[macro_export]
macro_rules! input_str {
    ($name:expr) => {
        include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/", $name))
    }
}

/// Load an input file.
#[macro_export]
macro_rules! input {
    ($name:expr) => {
        std::io::Cursor::new(input_str!($name))
    }
}

/// Read input as a long set of columns.
#[macro_export]
macro_rules! columns {
    ($name:expr, $sep:expr, $ty:ty) => {
        input_str!($name).trim()
            .split($sep)
            .filter(|s| !s.is_empty())
            .map(str::parse)
            .collect::<Result<Vec<$ty>, _>>()
    }
}

/// Read and parse lines.
#[macro_export]
macro_rules! lines {
    ($data:expr, $($ty:ty),*) => {{
        struct Iter<R, N> {
            data: R,
            i: N,
            line: String,
        }

        impl<R, N> Iterator for Iter<R, N> where R: std::io::BufRead, N: Iterator<Item = usize> {
            type Item = ($($ty,)*);

            fn next(&mut self) -> Option<Self::Item> {
                let size = self.data.read_line(&mut self.line)
                    .expect("failed to read line");

                if size == 0 {
                    return None;
                }

                let mut it = self.line.split_whitespace();

                let i = self.i.next().unwrap();
                let mut n = 1..;

                let out = ($({
                    let n = n.next().unwrap();

                    it.next()
                        .ok_or_else(|| format!("missing column"))
                        .and_then(|p| str::parse::<$ty>(p).map_err(|e| format!("bad value `{}`: {}", p, e)))
                        .map_err(|e| format!("bad `{}` on {}:{}: {}", stringify!($ty), i, n, e))
                        .expect("bad line")
                },)*);

                self.line.clear();
                Some(out)
            }
        }

        Iter {
            data: $data,
            i: 1..,
            line: String::new(),
        }
    }}
}
