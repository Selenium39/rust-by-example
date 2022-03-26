fn main() {
    let mut n = 0u32;
    loop {
        n += 1;
        if n == 5 {
            continue;
        };
        if n == 10 {
            break;
        };
        println!("{}", n);
    }

    // 嵌套循环,必须使用'label注明
    'out_loop: loop {
        'inner_loop: loop {
            println!("inner_loop");
            // 终止内部循环
            // break;
            // 终止外部循环
            break 'out_loop;
        }
        println!("out_loop")
    }

    // 循环可以有返回值,放在break后面
    let mut sum = 0;
    let sum = loop {
        sum += 1;
        if sum == 20 {
            break sum;
        }
    };
    assert_eq!(sum, 20);

    // for 循环
    for i in 1..11 {
        println!("{}", i);
    }

    let names = vec!["foo", "bar", "Rustacean"];
    // iter()：在每次迭代中借用元素，不会改变原集合元素
    for name in names.iter() {
        match name {
            &"Rustacean" => {
                println!("i am a {}", name)
            }
            _ => {
                println!("hello,{}", name)
            }
        }
    }
    println!("{:?}", names);

    // into_iter(): 每次迭代都会消耗集合中的元素
    let cars = vec!["benz", "audio", "baoma"];
    for car in cars.into_iter() {
        println!("{}", car);
    }
    // println!("cars:{:?}", cars);

    // iter_mut()：可变的借用某个元素，会改变原集合元素
    let mut fruits = vec!["apple", "orange", "balana"];
    for fruit in fruits.iter_mut() {
        *fruit = "good";
    }
    println!("{:?}", fruits);

    let boolean = true;
    match boolean {
        true => println!("is true"),
        _ => println!("is not true"),
    }

    let tuple = (1, 2, 3);
    match tuple {
        // match 解构赋值
        (x, y, z) => println!("x:{},y:{},z:{}", x, y, z),
    }

    enum Color {
        Red,
        Blue,
    };

    let red = Color::Red;
    match red {
        // match 解构枚举
        Color::Red => println!("red!!!"),
        _ => println!("not red"),
    }

    let val = &4;
    match val {
        // match匹配引用值
        &val => println!("{}", val),
    }
    // 先解引用了再匹配
    match *val {
        val => println!("{}", val),
    }

    // match 加上判断
    let pair = (2, -2);
    match pair {
        (x, y) if x == y => println!("eql"),
        (x, y) if x + y == 0 => println!("reverse number"),
        _ => println!("other"),
    }
    // if let
    let number = Some(7);
    if let Some(i) = number {
        println!("i is {}", i);
    }

    let letter: Option<i32> = None;
    if let Some(i) = letter {
        println!("letter i is {}", i);
    } else {
        println!("i is none");
    }

    // while let
    let mut opt = Some(0);
    while let Some(i) = opt {
        if i > 9 {
            println!("i is bigger than 9");
            opt = None;
        } else {
            println!("i is small than 9");
            opt = Some(i + 1);
        }
    }
}
