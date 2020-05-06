#[derive(Debug)]
pub struct Args {
    pub command: String,
    pub sub_command: Option<String>,
}

impl Args {
    pub fn new(args: &[String]) -> Result<Self, &'static str> {
        if args.len() < 2 {
            return Err("not enough arguments");
        }
        let command = args[1].clone();
        let sub_command = if args.len() > 2 {
            Some(args[2].clone())
        } else {
            None
        };

        Ok(Args { command, sub_command })
    }
}
