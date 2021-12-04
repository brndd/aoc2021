use std::io;
use std::io::BufRead;

extern crate circular_queue;
use circular_queue::CircularQueue;

fn main() {
    println!("{}", calculate().to_string());
}

fn calculate() -> i32 {
    let mut buffer = CircularQueue::<i32>::with_capacity(3);
    let mut last: Option<i32> = None;
    let mut changes = 0;

    let stdin = io::stdin();

    for line in stdin.lock().lines() {
        match line {
            Ok(text) => {
                let n = parse_int(text);
                buffer.push(n);
                
                if !buffer.is_full() {
                    continue;
                }
                let sum = buffer.iter().sum::<i32>();
                
                if let Some(m) = last {
                    if sum > m {
                        changes += 1;
                    }
                }
                last = Some(sum);
            },
            Err(_) => continue,
        }
    }

    return changes;
}

fn parse_int(text: String) -> i32 {
    match text.parse::<i32>() {
        Ok(n) => return n,
        Err(_) => return 0,
    }
}
