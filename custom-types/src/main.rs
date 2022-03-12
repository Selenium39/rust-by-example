// 隐藏对未使用代码的警告
#![allow(dead_code)]

use List::*;

//单元结构体
struct UnitStruct;

// 元组结构体
struct TupleStruct(i32, i32);

// c语言风格结构体
#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

// 结构体可以作为另外一个结构体的字段
#[derive(Debug)]
struct Rectangle {
    left_top: Point,
    right_bottom: Point,
}

impl Rectangle {
    fn rect_area(&mut self) -> i32 {
        let Point { x: x1, y: y1 } = self.left_top;
        let Point { x: x2, y: y2 } = self.right_bottom;
        (x2 - x1) * (y2 - y1)
    }
}

fn square(point: Point, length: i32) -> Rectangle {
    let Point { x, y } = point;
    Rectangle {
        left_top: point,
        right_bottom: Point {
            x: x + length,
            y: y + length,
        },
    }
}

enum Sex {
    Male,
    Female,
}

enum VeryLongEnumType123456 {}

// 可以给长枚举起别名
type shortEnum = VeryLongEnumType123456;

fn judgeSex(sex: Sex) {
    match sex {
        Sex::Male => println!("is Male"),
        Sex::Female => println!("is Female"),
    }
}

enum Status {
    Rich,
    Poor,
}

enum Work {
    Easy,
    Hard,
}

enum Number {
    Zero,
    One,
    Two,
}

enum Time {
    Second = 1,
    Hour = 3600,
}

enum List {
    // 元组结构体，包含一个元素和指向下一个元素的指针
    Cons(u32, Box<List>),
    //尾节点
    Nil,
}

impl List {
    fn new() -> List {
        Nil
    }

    fn prepend(self, element: u32) -> List {
        Cons(element, Box::new(self))
    }

    fn length(&self) -> u32 {
        match *self {
            // 递归
            Cons(_, ref tail) => tail.length() + 1,
            Nil => 0,
        }
    }

    fn stringify(&self) -> String {
        match *self {
            // format! 返回一个堆分配的字符串而不是打印在控制台
            Cons(head, ref tail) => format!("{}->{}", head, tail.stringify()),
            Nil => format!("Nil"),
        }
    }
}

static LANGUAGE: &'static str = "Rust";
const THRESHOLD: i32 = 32;

fn main() {
    let point: Point = Point { x: 1, y: 1 };
    println!("x:{},y:{}", point.x, point.y);
    // 使用结构体更新语法可以在原结构体的基础上创建新的结构体
    let point1: Point = Point { x: 2, ..point };
    println!("x:{},y:{}", point1.x, point1.y);
    // 解构赋值
    let Point { x: _, y: _ } = point1;
    println!("x:{},y:{}", point1.x, point1.y);
    // 实例化一个单元结构体
    let _unit = UnitStruct;
    // 实例化一个元组结构体
    let tuple = TupleStruct(1, 1);
    println!("{},{}", tuple.0, tuple.1);
    let mut rectangle = Rectangle {
        left_top: Point { x: 0, y: 0 },
        right_bottom: Point { x: 5, y: 5 },
    };
    println!("{:?}", rectangle);
    println!("area is {}", rectangle.rect_area());
    println!("square is {:?}", square(point, 5));
    judgeSex(Sex::Male);
    // 使用use声明后可以使用不完整的路径
    use Status::{Poor, Rich};
    use Work::*;
    // 枚举拥有从0开始的隐式值
    println!("{}", Number::Zero as i32);
    // 也可以给枚举显式赋值
    println!("{}", Time::Hour as i32);

    // 链表
    let mut list = List::new();
    list = list.prepend(1);
    list = list.prepend(2);
    list = list.prepend(3);
    println!("list length is {}", list.length());
    println!("list:{}", list.stringify());
    println!("{}", LANGUAGE);
}
