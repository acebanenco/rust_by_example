extern crate core;

use std::env;
use std::time::Instant;

use sha2::{Digest, Sha256};

use tokio;
use tokio::task;
use tokio::task::JoinHandle;

#[tokio::main]
async fn main() {
    let args:Vec<String> = env::args().collect();

    let start = Instant::now();

    let mut max_number = 1_000_000;
    if args.len() == 2 {
        max_number = args[1].parse::<usize>().unwrap();
    }

    let batch_size = 100_000;
    let tasks_count = max_number / batch_size;

    let mut fs = Vec::with_capacity(tasks_count);

    let mut offset = 0;
    for _i in 0..tasks_count {
        let fut = task::spawn(get_match_count_async(offset, batch_size));
        fs.push(fut);
        offset += batch_size;
    }

    let total_count = get_total_count(&mut fs).await;

    let duration = start.elapsed();
    println!("Found total {} mathces in {:.2?}", total_count, duration);
}

async fn get_total_count(fs: &mut Vec<JoinHandle<usize>>) -> usize {
    let mut total_count: usize = 0;
    while fs.len() > 0 {
        let fut = fs.remove(0);
        let count = fut.await.unwrap();
        total_count += count;
    }
    total_count
}

async fn get_match_count_async(from: usize, batch_size: usize) -> usize {
    return get_match_count(from, batch_size);
}

fn get_match_count(from: usize, batch_size: usize) -> usize {
    let mut hasher: Sha256 = Sha256::new();

    let mut msg_vec = [0_u8; 4];
    let mut count = 0;

    for index in from..from + batch_size {
        msg_vec[0] = (index >> 24) as u8;
        msg_vec[1] = (index >> 16) as u8;
        msg_vec[2] = (index >> 8) as u8;
        msg_vec[3] = (index >> 0) as u8;

        hasher.update(&msg_vec);
        let output = hasher.finalize_reset();

        if output[0] == 0 {
            count = count + 1;
        }
    }
    count
}


