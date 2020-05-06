extern crate dirs;
extern crate toml;

use std::fs;
use std::process::Command;
use std::path::Path;
use toml::{map::Map, Value};

#[derive(Debug)]
pub struct Config {
    pub user_name: String,
    pub password: String,
    pub workspace: String,
    pub repository_name: String,
}

impl Config {
    pub fn read() -> Result<Self, &'static str> {
        // configファイルが存在しなかったらエラー
        if !Path::new(&Config::file()).exists() {
            return Err("You must complete the setup before. Please execute \"bb config\"");
        }

        // configファイルから設定値を取得
        let toml_str = fs::read_to_string(&Config::file()).unwrap();
        let value: Value = toml::from_str(&toml_str).unwrap();
        let conf = value.get("default").unwrap();
        let user_name = conf.get("user_name").unwrap().to_string();
        let password = conf.get("password").unwrap().to_string();
        let workspace = conf.get("workspace").unwrap().to_string();

        let repository_name =  Config::read_repository_name();

        Ok(Config {
            user_name,
            password,
            workspace,
            repository_name,
        })
    }

    fn dir() -> String {
        // $HOME/.config/bb
        let home = dirs::home_dir().unwrap();
        format!("{}/.config/bb", home.to_str().unwrap())
    }

    fn file() -> String {
        format!("{}/config", Config::dir())
    }

    pub fn create(user_name: String, password: String, workspace: String) {
        let mut map = Map::new();
        let mut table = Map::new();
        map.insert("user_name".into(), Value::String(user_name));
        map.insert("password".into(), Value::String(password));
        map.insert("workspace".into(), Value::String(workspace));
        table.insert("default".into(), Value::Table(map));

        let toml_str = toml::to_string(&table).unwrap();

        // ディレクトリ作成
        if !Path::new(&Config::dir()).exists() {
            fs::create_dir_all(&Config::dir()).unwrap();
        }
        // ファイル作成
        fs::write(Config::file(), toml_str).unwrap();
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
