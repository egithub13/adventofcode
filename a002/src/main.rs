use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() -> io::Result<()> {
    let file = File::open("list.txt")?;
    let reader = BufReader::new(file);

    let mut report: Vec<i32> = Vec::new();
    let mut safe_reports = 0;

    for line_result in reader.lines() {
        let line = line_result?;
        // split on any whitespace, skip empty tokens
        for levels in line.split_whitespace() {
            report.push(levels.parse().expect("Failed to parse the number"));
        }
        if safe_or_unsafe(&report) {
            safe_reports += 1;
        } else {
            eprintln!("Unsafe report: {:?}", report);
        }
        report.clear(); // Clear the report for the next line
    }
    println!("Number of safe reports: {}", safe_reports);
    //println!("{}", increasing_or_decreasing(&report));
    Ok(())
    
}


fn safe_or_unsafe(report: &Vec<i32>) -> bool {
    let mut increasing = true;
    let mut decreasing = true;

    for i in 1..report.len() {

        if (report[i] - report[i - 1]).abs() > 3 || (report[i] - report[i - 1]).abs() < 1 {
            return false; 
        }

        if report[i] < report[i - 1] {
            increasing = false;
        }
        if report[i] > report[i - 1] {
            decreasing = false;
        }
    }

    increasing || decreasing
}   
