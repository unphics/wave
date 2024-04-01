// vscode插件推荐:
// 1. rust-analyzer
// 2. rust syntax: 语法高亮
// 3. crates: 分析当前项目的依赖是否是最新的版本
// 4. better toml: rust使用toml做项目的配置管理, better toml可以语法高亮并展示文件中的错误
// 5. rust test lens: 快速运行某个rust测试
// 6. tabnine: 基于ai的自动补全(tabnine enterprise是其企业版)

use std::{fs};

fn apply(v: i32, f: fn(i32)->i32) -> i32 {
    return f(v);
}
fn square(v: i32) -> i32 {
    return v * v;
}
fn cube(v: i32) -> i32 {
    v * v * v
}

#[derive(Debug)]
enum Gender {
    Unspecified = 0,
    Female = 1,
    Male = 2,
}
#[derive(Debug, Copy, Clone)]
struct UserId(u64);
#[derive(Debug, Copy, Clone)]
struct TopicId(u64); // 主题,话题
#[derive(Debug)]
struct User {
    Id: UserId,
    Name: String,
    Gender: Gender,
}
#[derive(Debug)]
struct Topic {
    Id: TopicId,
    Name: String,
    Owner: UserId,
}
// 定义聊天室中可能发生的事件
#[derive(Debug)]
enum Event {
    Join((UserId, TopicId)),
    Leave((UserId, TopicId)),
    Message((UserId, TopicId, String)),
}

fn fib_loop(n: u8) {
    println!("fib_loop call, n = {}", n);
    let mut a = 1;
    let mut b = 1;
    let mut i = 2u8;

    loop {
        let c = a + b;
        a = b;
        b = c;
        i += 1;
        println!("next val is {}", b);
        if i >= n {
            break;
        }
    }
}
fn fib_while(n: u8) {
    println!("fib_while call, n = {}", n);
    let (mut a, mut b, mut i) = (1, 1, 2);
    while i < n {
        let c = a + b;
        a = b;
        b = c;
        i += 1;
        println!("next val is {}", b)
    }
}
fn fib_for(n: u8) {
    println!("fib_for call, n = {}", n);
    let (mut a, mut b) = (1, 1);
    for _i in 2..n {
        let c = a + b;
        a = b;
        b = c;
        println!("next val is {}", b);
    }
}

fn process_event(event: &Event) {
    match event {
        Event::Join((uid, _tid)) => println!("user {:?} joined", uid),
        Event::Leave((uid, tid)) => println!("user {:?} left {:?}", uid, tid),
        Event::Message((_, _, msg)) => println!("broadcast: {}", msg),
    }
}
// 如果只关心Message可以这样写:
fn process_message(event: &Event) {
    if let Event::Message((_,_,msg)) = event {
        println!("broadcast: {}", msg);
    }
}

pub fn ch03_first() {

    /* 一个实用的rust程序 */

    let url = "https://www.rust-lang.org/";
    let output = "rust.md";

    println!("fetch url: {}", url);
    let body = reqwest::blocking::get(url).unwrap().text().unwrap();

    println!("convert html to markdown...");
    let md = html2md::parse_html(&body);

    fs::write(output, md.as_bytes()).unwrap();
    println!("converted markdown has been saved in {}", output);

    /* 变量和函数 */

    println!("apply square: {}", apply(2, square));
    println!("apply cube: {}", apply(2, cube));

    /* 一个聊天服务的数据结构 */
    let alice = User { Id: UserId(1), Name: "Alice".into(), Gender: Gender::Female };
    let bob = User { Id: UserId(2), Name: "Bob".into(), Gender: Gender::Male };
    let topic = Topic { Id: TopicId(1), Name: "rust".into(), Owner: UserId(1) };
    let event1 = Event::Join((alice.Id, topic.Id));
    let event2 = Event::Join((bob.Id, topic.Id));
    let event3 = Event::Message((alice.Id, topic.Id, "Hello world!".into()));
    println!("event1: {:?}, event2: {:?}, event3: {:?}", event1, event2, event3);

    /* 流程控制 斐波那契数列 */

    let n = 10;
    fib_loop(n);
    fib_while(n);
    fib_for(n);

    /* 模式匹配 */
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}