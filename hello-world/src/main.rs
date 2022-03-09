use std::fmt;

struct Unprintable(i32);

#[derive(Debug)]
struct DebugPrintable(i32);

#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8,
}

struct DisplayPrintable(i32);

#[derive(Debug)]
struct Complex {
    real: f32,
    imag: f32,
}

// 手动实现Display方法，任何非泛型的容器类型，都能实现Display方法
impl fmt::Display for DisplayPrintable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} + {}i", self.real, self.imag)
    }
}

struct List(Vec<i32>);

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[")?;
        for (count, value) in self.0.iter().enumerate() {
            if count != 0 {
                write!(f, ",")?;
            }
            write!(f, "{}", value)?;
        }
        write!(f, "]")
    }
}

struct List1(Vec<i32>);

impl fmt::Display for List1 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let vec = &self.0;
        write!(f, "[")?;
        for (index, value) in vec.iter().enumerate() {
            if index != 0 {
                write!(f, ",")?;
            }
            write!(f, "{}:{}", index, value)?;
        }
        write!(f, "]")
    }
}

#[derive(Debug)]
struct RGB {
    red: u8,
    green: u8,
    blue: u8,
}

impl fmt::Display for RGB {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{:?} 0x{:02X}{:02X}{:02X}",
            self, self.red, self.green, self.blue
        )
    }
}

fn main() {
    println!("{:?}", DebugPrintable(3));
    // 美化打印
    println!(
        "{:#?}",
        Person {
            name: "selenium39",
            age: 24
        }
    );
    let complex = Complex {
        real: 3.3,
        imag: 7.2,
    };
    println!("{}", DisplayPrintable(3));
    println!("Debug:{:?}", complex);
    println!("Display:{}", complex);
    println!("list:{}", List(vec![1, 2, 3]));
    println!("list1:{}", List1(vec![1, 2, 3]));
    for color in [
        RGB {
            red: 128,
            green: 255,
            blue: 90,
        },
        RGB {
            red: 0,
            green: 3,
            blue: 254,
        },
        RGB {
            red: 0,
            green: 0,
            blue: 0,
        },
    ]
    .iter()
    {
        println!("{}", color)
    }
}
