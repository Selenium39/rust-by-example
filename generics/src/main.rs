use std::fmt::{Debug, Display};
struct MultiValue<T>(T);
// 不可复制类型
struct Empty;
struct Null;

//需要在类型前写上<T>,说明T代表泛型
impl<T> MultiValue<T> {
    fn val(&self) -> &T {
        &self.0
    }
}

trait MyDrop<T> {
    fn drop(self, _: T);
}

// 为泛型的所有类型和调用者实现MyDrop<T>
impl<T, U> MyDrop<T> for U {
    fn drop(self, _: T) {}
}

// 通过+实现多重类型约束
fn my_print<T: Debug + Display>(t: &T) {
    println!("{}", t);
    println!("{:?}", t);
}

// 通过where语句进行类型约束
fn good_print<T>(t: &T)
where
    T: Debug + Display,
{
    println!("{}", t);
    println!("{:?}", t);
}

struct Container(i32, i32);

trait Contains {
    // 这里定义可以被方法使用的泛型类型（关联类型）
    type A;
    type B;
    fn first(&self) -> i32;
    fn last(&self) -> i32;
    fn contains(&self, _: &Self::A, _: &Self::B) -> bool;
}

impl Contains for Container {
    type A = i32;
    type B = i32;
    fn first(&self) -> i32 {
        self.0
    }
    fn last(&self) -> i32 {
        self.1
    }
    fn contains(&self, a: &i32, b: &i32) -> bool {
        &self.first() == a && &self.last() == b
    }
}

fn diffrence<C: Contains>(t: &C) -> i32 {
    t.last() - t.first()
}

fn main() {
    let a = MultiValue(1);
    let b = MultiValue('a');
    println!("a:{}", a.val());
    let empty = Empty;
    let null = Null;
    empty.drop(null);
    my_print(&String::from("rust"));
    good_print(&String::from("rust"));
    let container = Container(1, 9);
    println!("first:{}", container.first());
    println!("last:{}", container.last());
    println!("contains:{}", container.contains(&1, &9));
    println!("diffrence:{}", diffrence(&container));
}
