extern crate bb;

use std::env;
use std::process;
use bb::cmd::run;
use bb::config::Config;
use bb::args::Args;

fn main() {
    let args: Vec<String> = env::args().collect();
    let args = Args::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments {}", err);
        process::exit(1);
    });
    let config = Config::read();
    run(&args, &config);
}
