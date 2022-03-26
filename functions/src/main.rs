struct Point {
    x: f64,
    y: f64,
}

impl Point {
    //这两个方法都是静态方法，不需要实例调用，一般作为构造方法
    fn origin() -> Point {
        Point { x: 0.0, y: 0.0 }
    }

    fn new(x: f64, y: f64) -> Point {
        Point { x, y }
    }
}

struct Rectangle {
    p1: Point,
    p2: Point,
}

impl Rectangle {
    // 实例方法
    fn area(&self) -> f64 {
        ((self.p2.x - self.p1.x) * (self.p2.y - self.p1.y)).abs()
    }
    // 如果要修改对象本身属性，需要使用&mut self
    fn transform(&mut self, x: f64, y: f64) {
        self.p2.x += x;
        self.p2.y += y;
    }
}

struct Pair(Box<i32>, Box<i32>);

impl Pair {
    // self会消耗调用者的资源，所以只能调用一次，不能重复调用
    fn destory(self) {
        let Pair(first, second) = self;
        println!("first is {}, sencond is {}", first, second);
    }
}

// F必须为没有输入和返回值的函数
fn apply<F>(f: F)
where
    F: Fn(),
{
    f();
}

fn create_fn() -> impl Fn() {
    let str = "Fn".to_owned();
    move || println!("create_fn:{}", str)
}

fn main() {
    // 静态方法通过::调用
    let p1 = Point::origin();
    let p2 = Point::new(5.0, 5.0);
    let mut rectangle = Rectangle { p1, p2 };
    // 实例方法直接通过.调用
    let area = rectangle.area();
    println!("area is {}", area);
    rectangle.transform(5.0, 5.0);
    let area = rectangle.area();
    println!("after transform,area is {}", area);
    let pair = Pair(Box::new(5), Box::new(6));
    pair.destory();
    // pair.destory();

    // 闭包函数
    let add_closure = |x: i32| x + 1;
    let a = 10;
    println!("a add is {}", add_closure(a));

    let color = String::from("red");
    // 闭包会立即借用color变量，并将该借用储存到print中
    let print = || println!("color:{}", color);
    print();
    // 仍然可以被继续借用
    let color1 = &color;
    print();
    apply(print);
    create_fn()();
    let arr = vec![1, 2, 3];
    println!("arr include 2 ? {}", arr.iter().any(|&x| x == 2));
    println!("arr include 2 ? {}", arr.into_iter().any(|x| x == 2));
    let arr1 = vec![4, 5, 6];
    println!("arr1 find 4? {:?}", arr1.iter().find(|&&x| x == 4));
    println!("arr1 find4? {:?}", arr1.into_iter().find(|&x| x == 4))
}
