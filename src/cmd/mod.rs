use crate::config::Config;
use crate::args::Args;

mod pr;

pub fn run(args: &Args, config: &Config) {
    match args.command.as_str() {
        "pr" => pr(args, config),
        _ => println!("存在しないコマンドが指定されました。"),
    }
}

fn pr(args: &Args, config: &Config) {
    match args.sub_command.as_str() {
        "list" => pr::list(config),
        _ => println!("存在しないサブコマンドが指定されました。"),
    }
}

fn config(config: &Config) {
    // ./config/bb のディレクトリ作成
    // user_name,password,workspaceを対話的に取得
    // .config/bb/config ファイルを作成
    // [default]
    // user_name=xxxx
    // password=xxxxx
    // workspace=xxxx
}