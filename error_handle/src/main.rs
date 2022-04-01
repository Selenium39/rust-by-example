use std::collections::HashMap;
use std::rc::Rc;
use std::sync::Arc;
use std::thread;

fn send_gift(gift: &str) {
    if gift == "snake" {
        panic!("aaa")
    }
    println!("i love gift:{}", gift);
}

fn send_gift1(gift: Option<&str>) {
    match gift {
        Some("snake") => println!("fuck"),
        Some(gift) => println!("good gift:{}", gift),
        None => println!("wuuwuw"),
    }
}

fn send_gift2(gift: Option<u32>) -> Option<u32> {
    // 使用?解开Option，有则返回，无则None
    let result: u32 = gift?;
    Some(result)
}

struct Food;
struct First(Food);
struct Second(Food);

fn eat(food: Option<Food>) -> Option<Second> {
    // 通过组合算子，解开Option
    food.map(|f| First(Food)).map(|First(Food)| Second(Food))
}
fn main() {
    // send_gift("snake");
    send_gift1(Some("snake"));
    println!("{:?}", send_gift2(Some(8)));
    // and_then相当于flatMap
    println!("{:?}", (Some(Some(9))).and_then(|x| x));
    let mut map = HashMap::new();
    map.insert("a", 1);
    map.insert("b", 2);

    let foo = "foo".to_string();
    {
        // 创建Rc对象(引用计数器)
        let rc_foo = Rc::new(foo);
        println!("{}", rc_foo);
        // 引用计数为1
        println!("{}", Rc::strong_count(&rc_foo));
        let rc_bar = Rc::clone(&rc_foo);
        // 克隆后引用计数+1
        println!("{}", Rc::strong_count(&rc_foo));
    }

    // 共享引用计数器
    let apple = Arc::new("An apple");

    for _ in 0..10 {
        let apple = Arc::clone(&apple);
        thread::spawn(move || {
            println!("apple {:?}", apple);
        });
    }
}
