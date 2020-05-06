extern crate bb;
extern crate dotenv;

use std::env;
use std::process;
use dotenv::dotenv;
use bb::cmd::run;
use bb::config::Config;
use bb::args::Args;

fn main() {
    dotenv().ok();

    let args: Vec<String> = env::args().collect();
    let args = Args::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments {}", err);
        process::exit(1);
    });
    let config = Config::read();
    run(&args, &config);
}
