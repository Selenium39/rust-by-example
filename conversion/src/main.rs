use std::convert::From;
use std::convert::TryFrom;
use std::result::Result;
use std::string::ToString;

#[derive(Debug)]
struct MyNumber {
    value: i32,
}

#[derive(Debug)]
struct MyNumber1(i32);

// From trait
// 实现From trait的同时也实现了Into trait
impl From<i32> for MyNumber {
    fn from(value: i32) -> MyNumber {
        MyNumber { value }
    }
}

// TryFrom用于易出错的转换，所以返回值也是Result
impl TryFrom<i32> for MyNumber1 {
    type Error = ();
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value % 2 == 0 {
            Ok(MyNumber1(value))
        } else {
            Err(())
        }
    }
}

struct Circle(u32);

impl ToString for Circle {
    fn to_string(&self) -> String {
        format!("circle radis is {}", self.0)
    }
}

fn main() {
    let num = MyNumber::from(777);
    println!("{:?}", num);
    let num2 = 888;
    let num3: MyNumber = num2.into();
    println!("{:?}", num3);
    let num4 = MyNumber1::try_from(998).unwrap();
    println!("{:?}", num4);
    let circle = Circle(10);
    println!("{}", circle.to_string());
    // 将字符串转为数字
    let num5: i32 = "10".parse().unwrap();
    let num6: i32 = "5".parse().unwrap();
    println!("{}+{}={}", num5, num6, num5 + num6);
}
