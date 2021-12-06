use std::io;
use std::io::BufRead;
use ansi_term::Colour::Green;

fn main() {
    let stdin = io::stdin();

    // Read winning numbers
    let first = stdin.lock().lines().next();
    let mut numbers: Vec<i32> = Vec::new();
    if let Some(t) = first {
        if let Ok(text) = t {
            for num in text.split(',') {
                match num.parse::<i32>() {
                    Ok(n) => numbers.push(n),
                    Err(_) => continue,
                }
            }
        }
    }

    // Read cards
    let mut cards: Vec<Vec<Vec<(i32, bool)>>> = Vec::new();
    for line in stdin.lock().lines() {
        if let Ok(text) = line {
            let trimmed = text.trim();
            if trimmed.is_empty() {
                let card: Vec<Vec<(i32, bool)>> = Vec::new();
                cards.push(card);
                continue;
            }

            let mut card_line: Vec<(i32, bool)> = Vec::new();
            for num in text.split(' ') {
                match num.parse::<i32>() {
                    Ok(n) => {
                        card_line.push((n, false));
                    },
                    Err(_) => continue,
                }
            }
            let lastcard = cards.last_mut().unwrap();
            lastcard.push(card_line);
        }
    }

    // Check cards
    let mut winnerfound = false;
    let mut remaining_cards = cards.clone();
    for (numnum, num) in numbers.iter().enumerate() {
        let mut new_cards: Vec<Vec<Vec<(i32, bool)>>> = Vec::new();
        for (cardnum, card) in remaining_cards.iter_mut().enumerate() {
            // STINKY rust forcing me to put this here because M-MUH MEMORY SAFETY
            let mut copy = card.clone();
            let mut cardwon = false;
            for (i, row) in card.iter_mut().enumerate() {
                for (j, n) in row.iter_mut().enumerate() {
                    if n.0 == *num {
                        n.1 = true;
                        copy[i][j].1 = true; // STINKY RUST!! STINKY!!!!
                        if check_card(i, j, &copy) {
                            cardwon = true;
                            if !winnerfound {
                                let sum = sum_unmarked(&copy);
                                let score = sum * num;
                                println!("Winner winner chicken dinner! Card number {} with a sum of {} and a score of {}, after drawing a {} (#{}).", cardnum+1, sum, score, num, numnum+1);
                                print_pretty_card(&copy);
                                winnerfound = true;
                            }
                        }
                    }
                }
            }
            if !cardwon { new_cards.push(copy) };
        }
        if new_cards.is_empty() {
            let lastcard = remaining_cards.last().unwrap();
            let sum = sum_unmarked(&lastcard);
            let score = sum * num;
            println!("And the \"Worst card of the day\" award goes to... Card number {} with a score of {} after FINALLY drawing {}. We only had to draw {} numbers to get here!", remaining_cards.len(), score, num, numnum);
            print_pretty_card(&lastcard);
            break;
        }
        remaining_cards = new_cards;
    }
}

fn check_card(i: usize, j: usize, card: &Vec<Vec<(i32, bool)>>) -> bool {
    let width = card[0].len();
    let height = card.len();

    // Check row
    let mut win = true;
    for k in 0..width {
        if !card[i][k].1 {
            win = false;
            break;
        }
    }
    if win { return true };

    // Check column
    win = true;
    for k in 0..height {
        if !card[k][j].1 {
            win = false;
            break;
        }
    }
    if win { return true };

    // // Check topleft-bottomright diagonal if applicable
    // if i == j && width == height {
    //     for k in 0..height {
    //         if !card[k][k].1 {
    //             win = false;
    //             break;
    //         }
    //     }
    // }

    // //Check topright-bottomleft diagonal if applicable
    // if i == (width - j - 1) && width == height {
    //     for k in 0..height {
    //         if !card[k][width - k - 1].1 {
    //             win =false;
    //             break;
    //         }
    //     }
    // }

    return win;
}

fn sum_unmarked(card: &Vec<Vec<(i32, bool)>>) -> i32 {
    let mut sum = 0;

    for line in card {
        for cell in line {
            if !cell.1 { sum += cell.0; };
            
        }
    }

    return sum;
}

fn print_pretty_card(card: &Vec<Vec<(i32, bool)>>) {
    for line in card {
        for cell in line {
            let string: String;
            let digits = cell.0.to_string().len();
            if cell.1 {
                string = Green.paint(cell.0.to_string()).to_string();
            }
            else {
                string = cell.0.to_string();
            }
            print!("{}{} ", if digits < 2 {" "} else {""}, string); //there shouldn't be any 3-digit or more numbers in the data so this is good enough
        }
        println!("");
    }
}
