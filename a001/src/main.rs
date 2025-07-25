use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() -> io::Result<()> {
    let file = File::open("list.txt")?;
    let reader = BufReader::new(file);

    let mut first_numbers = Vec::new();
    let mut second_numbers = Vec::new();

    for line_result in reader.lines() {
        let line = line_result?;
        // split on any whitespace, skip empty tokens
        let mut parts = line.split_whitespace();

        if let (Some(a_str), Some(b_str)) = (parts.next(), parts.next()) {
            let a: i32 = a_str
                .parse()
                .expect("Failed to parse the first number");
            let b: i32 = b_str
                .parse()
                .expect("Failed to parse the second number");

            first_numbers.push(a);
            second_numbers.push(b);
        } else {
            eprintln!("Skipping malformed line: {:?}", line);
        }
    }

    //println!("First column:  {:?}", first_numbers[0]);
    //println!("Second column: {:?}", second_numbers[0]);

    first_numbers.sort();
    second_numbers.sort();
    let mut total = 0;

    for (a, b) in first_numbers.iter().zip(second_numbers.iter()) {
        total += (a - b).abs();
    }
    println!("Total: {}", total);

    Ok(())
}
