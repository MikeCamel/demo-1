use rand::prelude::*;
//use std::io::Read;
use std::{thread, time};

fn main() {
    let wait_secs = 20;
    let mut rng = rand::thread_rng();
    let number: u32 = rng.gen();
    let string = format!("{:08x}", number);
    println!("Secret created - wait {} seconds!", wait_secs);
    let wait_time = time::Duration::from_secs(wait_secs);
    let now = time::Instant::now();
    thread::sleep(wait_time);
    assert!(now.elapsed() >= wait_time);

    //println!("Press any key to reveal the secret...");
    //std::io::stdin().read_exact(&mut [0u8]).unwrap();

    println!("The secret is: {}", string);
}
