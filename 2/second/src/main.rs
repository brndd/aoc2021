use std::io;
use std::io::BufRead;

fn main() {
    let mut depth: i32 = 0;
    let mut distance: i32 = 0;
    let mut aim: i32 = 0;

    let stdin = io::stdin();

    for line in stdin.lock().lines() {
        if let Ok(text) = line {
            let words: Vec<&str> = text.split(' ').collect();
            let command: &str = words[0];
            let value: i32;
            match words[1].parse::<i32>() {
                Ok(n) => value = n,
                Err(_) => continue,
            }

            match command {
                "forward" => {
                    distance += value;
                    depth += aim * value;
                },
                "down" => aim += value,
                "up" => aim -= value,
                _ => continue,
            }
        }
    }

    println!("Depth: {} | Distance: {}", depth, distance);
    println!("Solution: {}", depth * distance);
}
