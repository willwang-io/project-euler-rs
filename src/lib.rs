use std::{fs, io, path::Path};

pub mod big_int;
pub mod linear_algebra;
pub mod number_theory;
pub mod utility;

pub fn fetch_input(filename: &str) -> io::Result<String> {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("input")
        .join(filename);
    fs::read_to_string(path)
}
