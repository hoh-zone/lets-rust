use std::fs::File;
use std::io::Read;
use std::num::ParseIntError;

fn read(src_path: &str) -> Result<Vec<String>, std::io::Error> {
    let mut src_file = File::open(src_path)?;
    let mut data = String::new();
    src_file.read_to_string(&mut data)?;

    let lines: Vec<String> = data.trim().split('\n').map(|s| s.to_string()).collect();

    Ok(lines)
}

fn sum(vals: Vec<String>) -> Result<u32, ParseIntError> {
    let mut total = 0;
    for v in vals {
        let num: u32 = v.parse()?;
        total += num;
    }
    Ok(total)
}

pub fn parse_and_sum(src_path: &str) -> Result<u32, Box<dyn std::error::Error>> {
    let vals = read(src_path)?;
    let total = sum(vals)?;
    Ok(total)
}