#[derive(Debug)]
pub struct Args {
    pub command: String,
    pub sub_command: String,
}

impl Args {
    pub fn new(args: &[String]) -> Result<Self, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let command = args[1].clone();
        let sub_command = args[2].clone();

        Ok(Args { command, sub_command })
    }
}
