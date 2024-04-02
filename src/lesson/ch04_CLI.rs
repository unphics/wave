use clap::{AppSetting, Clap};

// 定义HTTPie的CLI主入口， 他包含若干子命令
// 下面///为文档注释, clap会将其作为CLI的帮助

#[derive(Clap, Debug)]
#[clap(version = "1.0", author = "zys")]
#[clap(setting = AppSettings::ColorHelp)]
struct Opts {
    #[clap(subcommand)]
    subcmd: SubCommand,
}

// 子命令分别对应不同的HTTP方法， 目前只支持get /post
#[derive(Clap, Debug)]
enum SubCommand {
    Get(Get),
    Post(Post),
    // 暂且不支持其他HTTP方法
}

// get子命令
#[derive(Clap, Debug)]
struct Get {
    // http请求的url
    url: String,
}

// post子命令, 需要输入一个url, 和若干个可选的key-value, 用于json body
#[derive(Clap, Debug)]
struct Post {
    url: String,
    body: Vec<String>,
}

pub fn ch04_CLI() {
    let opts: Opts = Opts::parse();
    println!("{:?}", opts);
}