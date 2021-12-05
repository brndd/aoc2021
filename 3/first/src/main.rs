use std::io;
use std::io::BufRead;

fn main() {
    let mut bits: Vec<i32> = Vec::new();

    let stdin = io::stdin();

    for line in stdin.lock().lines() {
        if let Ok(text) = line {
            for (i, c) in text.chars().enumerate() {
                let num: i32 = if c == '1' { 1 } else { -1 };
                if bits.len() <= i {
                    bits.push(num);
                }
                else {
                    bits[i] += num;
                }
            }
        }
    }

    let mut gamma: u32 = 0;
    let length = bits.len();
    for (i, delta) in bits.iter().enumerate() {
        // length - 1 - i
        if delta < &0 {
            continue;
        }
        gamma = gamma | (1 << (length - i - 1));
    }
    let mut epsilon = !gamma;
    epsilon = epsilon << (32 - length) >> (32 - length);
    println!("Gamma: {} | Epsilon: {}", gamma, epsilon);
    println!("solution: {}", gamma * epsilon);
}
// 0 0 0 1 0 0 0 0