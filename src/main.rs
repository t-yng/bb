extern crate bb;
extern crate dotenv;

use std::env;
use std::process;
use dotenv::dotenv;
use bb::{Args, Config, run};

fn main() {
    dotenv().ok();

    let args: Vec<String> = env::args().collect();
    let args = Args::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments {}", err);
        process::exit(1);
    });
    let config = Config::new();
    run(&args, &config);
}
