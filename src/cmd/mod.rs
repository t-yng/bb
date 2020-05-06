use std::io::{self, Write};
use crate::config::Config;
use crate::args::Args;

mod pr;

pub fn run(args: &Args, conf: &Config) {
    match args.command.as_str() {
        "pr" => pr(args, conf),
        "config" => config(),
        _ => println!("存在しないコマンドが指定されました。"),
    }
}

fn pr(args: &Args, config: &Config) {
    let sub_command = &args.sub_command.as_ref().expect("サブコマンドが指定されていません");
    match sub_command.as_str() {
        "list" => pr::list(config),
        _ => println!("存在しないサブコマンドが指定されました。"),
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