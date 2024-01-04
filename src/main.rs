use strum_macros::EnumString;
use clap::Parser;
use rand::thread_rng;
use rand::seq::SliceRandom;
use rayon::prelude::*;

#[derive(Clone, EnumString)]
enum Strategy {
    Random,
    NumberFollow
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    strategy: Strategy,

    #[arg(short, long)]
    count: u32,

    #[arg(short, long)]
    num_of_prisoners: usize,
}

#[derive(Debug)]
struct Results {
    pass: u32,
    fail: u32,
}


fn main() {
    let args = Args::parse();

    let results = run_simulation(&args);

    println!("Results: {:?}", results);
    println!("% Success: {}", (results.pass * 100) as f32 / (results.pass + results.fail) as f32);
}

fn run_simulation(Args { strategy, count, num_of_prisoners }: &Args) -> Results {
    let mut results = Results {
        pass: 0,
        fail: 0,
    };
    for _ in 0..*count {
        let success = try_strategy(strategy, *num_of_prisoners);
        match success {
            true => {
                results.pass += 1;
            },
            false => {
                results.fail += 1;
            }
        }
    }
    results
}

fn try_strategy(strategy: &Strategy, num_of_prisoners: usize) -> bool {
    let boxes = generate_boxes(num_of_prisoners);
    let num_attempts: usize = num_of_prisoners / 2;
    (0..num_of_prisoners).into_par_iter().map(|i| match strategy {
        Strategy::Random => try_random(i, &boxes, num_attempts, num_of_prisoners),
        Strategy::NumberFollow => try_number_follow(i, &boxes, num_attempts),
    }).reduce(|| true, |acc, e| acc && e)
}

fn try_random(prisoner_num: usize, boxes: &Vec<usize>, num_attempts: usize, num_of_prisoners: usize) -> bool {
    let mut selected_boxes: Vec<usize> = (0..num_of_prisoners).collect();
    selected_boxes.shuffle(&mut thread_rng());
    selected_boxes.truncate(num_attempts);

    match selected_boxes.iter().map(|x| boxes[*x] == prisoner_num).reduce(|acc, e| acc || e) {
        Some(i) => i,
        None => false,
    }
}

fn try_number_follow(prisoner_num: usize, boxes: &Vec<usize>, num_attempts: usize) -> bool {
    let mut next_box = prisoner_num;

    for _ in 0..num_attempts {
        if boxes[next_box] == prisoner_num {
            return true
        } else {
            next_box = boxes[next_box];
        }
    }
    false
}

fn generate_boxes(num_prisoners: usize) -> Vec<usize> {
    let mut boxes: Vec<usize> = (0..num_prisoners).collect();
    boxes.shuffle(&mut thread_rng());
    boxes
}
