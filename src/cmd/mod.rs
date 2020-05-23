use std::io::{self, Write};
use crate::config::Config;
use crate::args::Args;

mod pr;

pub fn run(args: &Args) -> Result<(), String> {
    let command = args.command.as_str();

    // configコマンド実行時は設定ファイルが存在しない事もあるので先に処理する
    if command == "config" {
        return Ok(config());
    }

    let conf = match Config::read() {
        Ok(c) => c,
        Err(err) => return Err(err.to_string())
    };

    match command {
        "pr" => pr(args, &conf),
        _ => Err(format!("{} is not exists.", command)),
    }
}

fn pr(args: &Args, config: &Config) -> Result<(), String> {
    let sub_command = match args.sub_command.as_ref() {
        Some(v) => v,
        None => return Err("You must set the sub-command for pr.".to_string()),
    };

    match sub_command.as_str() {
        "list" => Ok(pr::list(config)),
        _ => Err(format!("{} is not exists for pr.", sub_command)),
    }
}

fn config() {
    // user_name,password,workspaceを対話的に取得
    let user_name = get_input("user_name: ");
    let password = get_input("password: ");
    let workspace = get_input("workspace: ");

    // .config/bb/config ファイルを作成
    Config::create(user_name, password, workspace);
}

fn get_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => {},
        Err(_e) => {},
    }

    input.trim_end().to_string()
}