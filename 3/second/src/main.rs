use std::io;
use std::io::BufRead;

fn main() {
    let mut numbers: Vec<u32> = Vec::new();
    let mut len: usize = 32;
    let stdin = io::stdin();

    for line in stdin.lock().lines() {
        if let Ok(text) = line {
            len = text.len();
            let conv = if let Ok(n) = u32::from_str_radix(&text, 2) { n } else { 0 };
            numbers.push(conv);
        }
    }

    let mut deltas = calculate_deltas(len, &numbers);

    let mut i = len;
    let mut o2numbers = numbers.clone();
    while i > 0 && o2numbers.len() > 1 {
        let commonest: u32;
        let delta = deltas[len - i];
        if delta >= 0 {
            commonest = 1;
        }
        else {
            commonest = 0;
        }
        o2numbers.retain(|&x| filter_bindigit(i-1, commonest, x));
        deltas = calculate_deltas(len, &o2numbers);
        i -= 1;
    }

    i = len;
    let mut co2numbers = numbers.clone();
    while i > 0 && co2numbers.len() > 1 {
        let uncommonest: u32;
        let delta = deltas[len - i];
        if delta >= 0 {
            uncommonest = 0;
        }
        else {
            uncommonest = 1;
        }
        co2numbers.retain(|&x| filter_bindigit(i-1, uncommonest, x));
        deltas = calculate_deltas(len, &co2numbers);
        i -= 1;
    }

    println!("O2: {} | CO2: {}", o2numbers[0], co2numbers[0]);
    println!("Result: {}", o2numbers[0] * co2numbers[0]);
}

fn calculate_deltas(numdigits: usize, numbers: &Vec<u32>) -> Vec<i32> {
    let mut deltas: Vec<i32> = Vec::new();
    
    for n in numbers.iter() {
        for i in (0..numdigits).rev() {
            let digit = 1 & (n >> i);
            let delta;
            if digit == 0 {
                delta = -1;
            }
            else {
                delta = 1;
            }

            if deltas.len() < numdigits {
                deltas.push(delta);
            }
            else {
                deltas[numdigits - 1 - i] += delta;
            }
        }
    }

    return deltas;
}

fn filter_bindigit(i: usize, cmpto: u32, digit: u32) -> bool {
    return 1 & (digit >> i) == cmpto;
}
