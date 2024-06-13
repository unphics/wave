pub mod gen {
    use my_macro::gen;
    gen!("my_macro/fixture/person.json");
}
use gen::*;
fn main() {
    
}