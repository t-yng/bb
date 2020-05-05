use std::env;
use std::process::Command;
use std::path::Path;

#[derive(Debug)]
pub struct Config {
    pub user_name: String,
    pub password: String,
    pub workspace: String,
    pub repository_name: String,
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
