struct Book {
    name: String
}

impl Book {
     fn show_book_name(&self) {
         // .... 
         println!("name is: {}", self.name);
     }
}

use std::ops::{Deref, DerefMut};
struct MyBook {
    p: Book,
    author: String,
}

impl MyBook {
    fn new() -> MyBook {
        MyBook {
            p: Book{name: "知乎大全".to_string()},
            author: "我是谁".to_string()
       }
     }
}

impl Deref for MyBook {
    type Target = Book;
    fn deref<'a>(&'a self) -> &'a Book {
          &self.p
    }
}
pub fn book() {
    let mut mybook = MyBook::new();
    mybook.show_book_name();

}