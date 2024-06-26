
use sqlite::{ReadableWithIndex, State};

use crate::error::ResultExt;
/**
 * @file data.rs
 * @brief sqlite的data_db的操作模块
 * @author zys
 * @data Thu May 02 2024 21:58:32 GMT+0800 (中国标准时间)
 * @version 0.1
 */
pub fn sqlite_test() {
    let conn = sqlite::open("path").expect("sqlite::open");
    let query = "
        insert info users values('10001', 'pwd');
    ";
    conn.execute(query).expect("conn.execute");
    // create table users(name TEXT, age INTEGER);
}

/**
 * 该表存在以此为主键的行
 */
pub fn exit_row(table_name: &str, main_key: i64) -> bool {
    let conn = sqlite::open("sqlite/wave_data.db").expect("sqlite::open");
    let query = format!("select count(*) from {} where account = ?", table_name);
    let mut statement = conn.prepare(query).expect("conn.prepare");
    statement.bind((1, main_key)).unwrap();
    if let Ok(State::Row) = statement.next() {
        let count = statement.read::<i64, _>(0).expect("statement.read");
        if count > 0 {
            return true;
        } else {
            return false;
        }
    } else {
        return false;
    }
}
pub fn insert_row<F>(table_name: &str, fmt_field_txt: &str, fmt_seat_txt: &str, f: F) -> bool where F: Fn(&mut sqlite::Statement){
    let conn: sqlite::Connection = sqlite::open("sqlite/wave_data.db").expect("sqlite::open");
    let query = format!("insert into {} ({}) values ({})", table_name, fmt_field_txt, fmt_seat_txt);
    let mut statement = conn.prepare(query).expect("insert_row conn.prepare");
    f(&mut statement);
    let res = statement.next().expect("");
    match res {
        State::Row => {
            return false;
        }
        State::Done => {
            return true;
        }
    }
}
/**
 * @brief 取表的行数
 */
pub fn get_row_count(table_name: &str) -> u32 {
    let conn: sqlite::Connection = sqlite::open("sqlite/wave_data.db").expect("sqlite::open");
    let query = format!("select count(*) from {}", table_name);
    let mut statement = conn.prepare(query).handle();
    let mut count = 0;
    if let Ok(State::Row) = statement.next() {
        count = statement.read::<i64, _>(0).handle();
    }
    return count as u32;
}
/**
 * @brief 读取字段的值
 */
pub fn read_field_str(table_name: &str, main_key_name: &str, main_key: i32, field_name: &str) -> String {
    let conn = sqlite::open("sqlite/wave_data.db").expect("sqlite::open");
    let query = format!("select {} from {} where {} = {}", field_name, table_name, main_key_name, main_key);
    let mut statement = conn.prepare(query).expect("conn.prepare");
    statement.next().unwrap();
    return statement.read::<String, _>(0).handle();
}
// trait from_sql{}
// impl from_sql for String {}
// impl from_sql for i32 {}
// impl from_sql for ReadableWithIndex {
    
// }
/**
 * @brief 修改字段的值
 */
pub fn modify_field_val(table_name: &str, main_key_name: &str, main_key: u32, field_name: &str, value: &str) {
    let conn: sqlite::Connection = sqlite::open("sqlite/wave_data.db").expect("sqlite::open");
    let query = format!("update {} set {} = {} where {} = {}", table_name, field_name, value, main_key_name, main_key);
    let mut statement = conn.prepare(query).handle();
    statement.next().handle();
}