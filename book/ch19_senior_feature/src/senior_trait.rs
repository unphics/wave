use std::ops::Add;
use std::fmt::{self, write};
use std::process::Output;

pub fn senior_trait() {
    // 1.在trait定义中使用关联类型来指定占位类型
    // 关联类型associated type是trait中的类型占位符, 他可以用于trait的方法签名中

    // 2. 默认泛型参数和运算符重载<PlaceholderType=ConcreteType>
    // 这种技术通常用于运算符重载, 虽然rust不允许创建自己的运算符及重载任意运算符
    // 但是可以通过实现std::ops中列出的那些trait来重载一部分相应的运算符
    assert_eq!(Point{x: 1, y: 0} + Point{x: 2, y: 3}, Point{x: 3, y: 3});

    // 3.完全限定语法(fully qualified syntax)
    // 如何调用同名方法
    let person = Human;
    person.fly(); // 此处会调用他本身的fly
    Pilot::fly(&person);
    Wizard::fly(&person);
    // 关联方法
    println!("a babby dog is called a {}", Dog::baby_name());
    // println!("a babby dog is called a {}", Animal::baby_name()); // 报错, 需要使用完全限定语法
    println!("a babby dog is called a {}", <Dog as Animal>::baby_name());
    // 3.2 使用supertrait来要求trait附带其他trait的功能
    // 3.3 使用newtype模式在外部类型上实现外部trait
    // 孤儿规则: 只有当trait或类型定义在本地包的时候, 才能为该类型实现这个trait
    // 可以使用newtype模式绕过该规则(使用tuple struct元组结构体创建一个新类型)
    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {}", w);
}

/// 1
trait iterator {
    type item;
    fn next(&mut self) -> Option<Self::item>;
}

// 关联类型和泛型的区别
// 泛型, 每次实现trait时需要标注类型, 可以为一个类型多次实现某个trait
// 关联类型, 无需标注类型, 不能为单个类型多次实现某个trait
// 泛型是静态多态, 关联类型只是类型占位
trait iterator2<T> {
    fn next(&mut self) ->Option<T>;
}
struct Counter {}
impl iterator for Counter {
    type item = u32;
    fn next(&mut self) -> Option<Self::item> {
        None
    }
}
impl iterator2<String> for Counter {
    fn next(&mut self) ->Option<String> {
        None
    }
}
impl iterator2<i32> for Counter {
    fn next(&mut self) ->Option<i32> {
        None
    }
}

/// 2.
#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}
impl Add for Point {
    type Output = Point;
    fn add(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}


/// 3
trait Pilot {
    fn fly(&self);
}
trait Wizard {
    fn fly(&self);
}
struct Human;
impl Pilot for Human {
    fn fly(&self) {
        println!("this is your captain speaking.");
    }
}
impl Wizard for Human {
    fn fly(&self) {
        println!("Up!")
    }
}
impl Human {
    fn fly(&self) {
        println!("*waving arms furiously*")
    }
}
trait Animal {
    fn baby_name() -> String;
}
struct Dog;
impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}
impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}
/// 3.2
trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        let Output = self.to_string();
        let len = Output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", Output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}
impl OutlinePrint for Point {} // 这个point的impl的outline_print是有默认的，但是还必须实现display
impl fmt::Display for Point{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
///3.3 new tpye
struct Wrapper(Vec<String>);
impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}