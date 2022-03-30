use std::ops;

struct Benz {
    name: String,
    price: i32,
}

trait Car {
    fn running(&self);
}

impl Car for Benz {
    fn running(&self) {
        println!("{} can running", self.name)
    }
}

struct Dog {}
struct Cat {}
trait Animal {
    fn bark(&self);
}
impl Animal for Dog {
    fn bark(&self) {
        println!("汪汪汪");
    }
}
impl Animal for Cat {
    fn bark(&self) {
        println!("喵喵喵")
    }
}

// 这里编译器不知道返回的是Dog还是Cat，需要使用dyn返回Trait
fn randomBark(number: i32) -> Box<dyn Animal> {
    if number & 2 == 0 {
        Box::new(Dog {})
    } else {
        Box::new(Cat {})
    }
}

struct A;
struct B;
#[derive(Debug)]
struct AB;

//运算符重载
impl ops::Add<B> for A {
    type Output = AB;
    fn add(self, rhs: B) -> AB {
        println!("A add B");
        AB
    }
}

struct Dropable {
    name: &'static str,
}

impl Drop for Dropable {
    // 对象离开作用域，会自动调用此方法
    fn drop(&mut self) {
        println!("drop:{}", &self.name)
    }
}

struct Fibonacci {
    curr: u32,
    next: u32,
}

impl Iterator for Fibonacci {
    type Item = u32;
    fn next(&mut self) -> Option<u32> {
        let new_next = self.curr + self.next;
        self.curr = self.next;
        self.next = new_next;
        Some(self.curr)
    }
}

#[derive(Debug, Clone)]
struct Person {
    name: &'static str,
}

struct Tom;

trait Mother {
    fn say1(&self);
}

trait Father {
    fn say2(&self);
}

// Rust没有继承，但是可以父Trait
trait Son: Mother + Father {
    fn say3(&self);
}

// 实现子Trait的同时还得实现父Trait
impl Son for Tom {
    fn say3(&self) {
        println!("say3");
    }
}
impl Mother for Tom {
    fn say1(&self) {
        println!("say3");
    }
}
impl Father for Tom {
    fn say2(&self) {
        println!("say3");
    }
}

trait Girl {
    fn fk(&self);
}

trait Boy {
    fn fk(&self);
}

struct Jeff;

impl Girl for Jeff {
    fn fk(&self) {
        println!("girl fk");
    }
}

impl Boy for Jeff {
    fn fk(&self) {
        println!("boy fk");
    }
}

fn main() {
    let car = Benz {
        name: String::from("benz"),
        price: 10,
    };
    car.running();
    let a = A;
    let b = B;
    a + b;
    let dropable = Dropable { name: "dy" };

    let mut iterators = 1..3;
    println!("{:?}", iterators.next());
    println!("{:?}", iterators.next());
    println!("{:?}", iterators.next());

    let mut f = Fibonacci { curr: 0, next: 1 };
    println!("{:?}", f.next());
    println!("{:?}", f.next());
    println!("{:?}", f.next());
    let person = Person { name: "selenium39" };
    let person1 = person.clone();
    // 这里失去了所有权
    // let person2 = person;
    // println!("{}", person.name);
    // clone不会使得person丧失所有权
    println!("{}", person.name);

    let jeff = Jeff;

    // 多个trait中有相同的方法，可以通过<Struct as Trait>::消除歧义
    <Jeff as Girl>::fk(&jeff);
    <Jeff as Boy>::fk(&jeff);
}
