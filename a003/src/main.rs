use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> std::io::Result<()> {
    // Open the file
    let file = File::open("list.txt")?;
    let reader = BufReader::new(file);

    // Regex to match mul(X,Y) with no spaces, where X and Y are 1-3 digit numbers
    let re = Regex::new(r"\bmul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let mut total = 0;
    // Read file line by line
    for line in reader.lines() {
        let line = line?; // Handle Result<String>
        for cap in re.captures_iter(&line) {
            let x: u32 = cap[1].parse().unwrap();
            let y: u32 = cap[2].parse().unwrap();

            total += x * y;
        }
    }
    // Print the total
    println!("Total: {}", total);
    Ok(())
}
