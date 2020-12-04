use rand::distributions::{Distribution, Uniform};
use std::env;

fn main() {
    match env::args().nth(1){
        Some(i) => println!("{}", to_sponge(&i)),
        None => println!("Please provide input to spongify")
    }
}

fn to_sponge(s: &str) -> String {
    let mut rng = rand::thread_rng();
    let d100 = Uniform::from(0..5);
    let mut r = String::new();
    let mut last_lower = Uniform::from(0..2).sample(&mut rng).eq(&1);
    for c in s.chars() {
        let lower: bool;
        if last_lower {
            if d100.sample(&mut rng) == 0 {
                lower = true;
            } else {
                lower = false;
            }
        } else {
            if d100.sample(&mut rng) != 0 {
                lower = true;
            } else {
                lower = false;
            }
        }
        if lower {
            for c2 in c.to_lowercase() {
                &r.push(c2);
            }
            last_lower = true;
        } else {
            for c2 in c.to_uppercase() {
                &r.push(c2);
            }
            last_lower = false;
        }
    }
    r
}
