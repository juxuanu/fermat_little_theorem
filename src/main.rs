use std::env::{self};

use rand::Rng;

fn main() {
    let mut n: usize = 10;
    let mut k: usize = 42;

    for (i, arg) in env::args().enumerate() {
        match i {
            0 => continue,
            1 => n = arg.parse::<usize>().expect("Provide a valid number for n"),
            2 => k = arg.parse::<usize>().expect("Provide a valid number for k"),
            _ => break,
        }
    }

    
    fermat_test(n, k)
}

fn fermat_test(n: usize, k: usize) {
    let mut rng = rand::thread_rng();

    if n <= 3 {
        return println!("{} is prime", n);
    }

    for _ in 1..k {
        let a = rng.gen_range(2..(n - 2));
        if usize::pow(a, (n - 1).try_into().unwrap()) % n != 1 {
            return println!("{} is composed", n);
        }
    }
    println!(
        "{} is prime with a probability of {}%",
        n,
        (1_f32 - f32::powf(0.5, k as f32)) * 100.0,
    );
}
