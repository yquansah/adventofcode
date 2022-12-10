use std::{fs::File, io::{BufReader, BufRead}};
use std::collections::BinaryHeap;

fn parse_file_and_solve(filename: &str) -> i32 {
    let file: File = File::open(filename).expect("open failed");
    let mut running_sum = 0;
    let reader  = BufReader::new(file);

    let mut heap = BinaryHeap::new();
    
    for line in reader.lines() {
        match line {
            Ok(v) => if v == "" {
                heap.push(running_sum);
                running_sum = 0;
            } else {
                running_sum += v.parse::<i32>().unwrap_or(0);
            },
            Err(_) => continue,
        }
    }

    if running_sum != 0 {
        heap.push(running_sum);
    }

    let mut top_3_cal = 0;
    for _ in 0..3 {
        match heap.pop() {
            Some(v) => {
                println!("v is {}", v);
                top_3_cal += v;
            },
            _ => continue,
        }
    }

    top_3_cal
}

fn main() {
    let max = parse_file_and_solve("input.txt");
    println!("The top 3 max is {}", max)
}
