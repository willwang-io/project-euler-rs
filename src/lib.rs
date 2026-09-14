use std::process::Command;

pub mod linear_algebra;
pub mod number_theory;

pub fn fetch_input(url: &str) -> Result<Vec<usize>, Box<dyn std::error::Error>> {
    let output = Command::new("curl").args(["-fLaS", url]).output()?;

    let text = String::from_utf8(output.stdout)?;

    let arr = text
        .trim()
        .split(",")
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    Ok(arr)
}
