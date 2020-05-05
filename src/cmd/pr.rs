extern crate reqwest;
extern crate serde;

use serde::{Deserialize};
use crate::config::Config;

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

pub fn list(config: &Config) {
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