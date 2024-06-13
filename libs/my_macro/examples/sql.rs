use my_macro::sql;
// cargo run --example sql
fn main() {
    sql!(select * from user where id = 1001);
}