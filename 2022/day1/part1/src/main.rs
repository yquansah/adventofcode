use std::{fs::File, io::{BufReader, BufRead}};

fn parse_file_and_solve(filename: &str) -> i32 {
    let file: File = File::open(filename).expect("open failed");

    let mut max = 0;
    let mut running_sum = 0;

    let reader  = BufReader::new(file);
    
    for line in reader.lines() {
        match line {
            Ok(v) => if v == "" {
                if running_sum > max {
                    max = running_sum;
                }
                running_sum = 0;
            } else {
                running_sum += v.parse::<i32>().unwrap_or(0);
            },
            Err(_) => continue,
        }
    }

    if running_sum > max {
        max = running_sum;
    }

    max
}

fn main() {
    let max = parse_file_and_solve("input.txt");
    println!("The max is {}", max)
}
