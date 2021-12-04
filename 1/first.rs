use std::io;
use std::io::BufRead;

fn main() {
    println!("{}", calculate().to_string());
}

fn calculate() -> i32 {
    let mut last;
    let mut changes = 0;

    let stdin = io::stdin();
    let first = stdin.lock().lines().next();
    match first {
        None => return 0,
        Some(t) => match t {
            Ok(text) => last = parse_int(text),
            Err(_) => return 0,  
        },
    }

    for line in stdin.lock().lines() {
        match line {
            Ok(text) => {
                let n = parse_int(text);
                if n > last {
                    changes += 1;
                }
                last = n;
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
