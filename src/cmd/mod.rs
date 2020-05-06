extern crate dirs;
extern crate toml;

use std::fs;
use std::io::{self, Write};
use crate::config::Config;
use crate::args::Args;
use toml::{map::Map, Value};

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
    // $HOME/.config/bb のディレクトリ作成
    let home = dirs::home_dir().unwrap();
    let dir = format!("{}/.config/bb", home.to_str().unwrap());
    fs::create_dir_all(&dir).unwrap();

    // user_name,password,workspaceを対話的に取得
    let user_name = get_input("user_name: ");
    let password = get_input("password: ");
    let workspace = get_input("workspace: ");

    // .config/bb/config ファイルを作成
    let mut map = Map::new();
    let mut table = Map::new();
    map.insert("user_name".into(), Value::String(user_name));
    map.insert("password".into(), Value::String(password));
    map.insert("workspace".into(), Value::String(workspace));
    table.insert("default".into(), Value::Table(map));

    let toml_str = toml::to_string(&table).unwrap();
    let file_path = format!("{}/config", &dir);
    fs::write(file_path, toml_str).unwrap();
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