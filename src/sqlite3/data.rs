use sqlite::State;
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