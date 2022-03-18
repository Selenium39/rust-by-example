fn main() {
    let decimal = 6.66666666;
    // 别名
    type aaaaaa = i32;

    // 不提供原生类型的隐式转换
    // let integer: i32 = decimal;

    // 可以显式的转换
    let integer: i32 = decimal as i32;
    println!("{}", integer);

    let x = 1i32;
    println!("{}", x);

    let y = 8i32;
    let mut vec = Vec::new();
    vec.push(y);
    // 类型推断为Vec<i32>
    println!("{:?}", vec);

    let b: aaaaaa = 1;
    println!("{}", b);
}
