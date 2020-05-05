extern crate reqwest;
extern crate serde;
extern crate toml;

use std::env;
use serde::{Deserialize};
use std::process::Command;
use std::path::Path;

#[derive(Debug)]
pub struct Args {
    command: String,
    sub_command: String,
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

#[derive(Debug)]
pub struct Config {
    user_name: String,
    password: String,
    workspace: String,
    repository_name: String,
}

impl Config {
    pub fn new() -> Self {
        // TODO: user_name, password, workspace はconfigコマンドで設定した値を読み取る
        let user_name = env::var("USER_NAME").unwrap();
        let password = env::var("PASSWORD").unwrap();
        let workspace =  env::var("WORKSPACE").unwrap();

        let repository_name =  Config::read_repository_name();

        Config {
            user_name,
            password,
            workspace,
            repository_name,
        }
    }

    fn read_repository_name() -> String {
        let url_output = Command::new("git")
            .args(&["config","--get", "remote.origin.url"])
            .output()
            .expect("リモートリポジトリ名の取得に失敗しました");
        let url = String::from_utf8(url_output.stdout).unwrap();
        let repository_name = Path::new(&url).file_name().unwrap();

        repository_name.to_str().unwrap().replace(".git", "").replace("\n", "")
    }
}

pub fn run(args: &Args, config: &Config) {
    match args.command.as_str() {
        "pr" => pr(args, config),
        _ => println!("存在しないコマンドが指定されました。"),
    }
}

fn pr(args: &Args, config: &Config) {
    match args.sub_command.as_str() {
        "list" => pr_list(config),
        _ => println!("存在しないサブコマンドが指定されました。"),
    }
}

#[derive(Deserialize, Debug)]
struct Branch {
    name: String,
}
#[derive(Deserialize, Debug)]
struct Source {
    branch: Branch,
}

#[derive(Deserialize, Debug)]
struct Value {
    source: Source,
}

#[derive(Deserialize, Debug)]
struct PullRequests {
    pagelen: i32,
    values: Vec<Value>,
}

fn pr_list(config: &Config) {
    let req_url = format!(
        "https://api.bitbucket.org/2.0/repositories/{}/{}/pullrequests?q=state=\"merged\"",
        &config.workspace,
        &config.repository_name
    );
    let client = reqwest::blocking::Client::new();
    let resp = client.get(req_url.as_str())
        .basic_auth(config.user_name.as_str(), Some(config.password.as_str()))
        .send()
        .unwrap();
    let result: PullRequests = resp.json().unwrap();
    let branche_names: Vec<&str> = result.values.iter()
        .map(|v| v.source.branch.name.as_str())
        .collect();
    println!("{}", branche_names.join("\n"));
}
