fn main() {
    // 变量默认不可变，通过mut标识可变变量
    let mut a = 1;
    a += 1;
    // 变量可以先声明再赋值
    let c;
    let d = 4;
    let d = d;
    // 被相同名称的变量绑定时，会被冻结
    // d = 5;
    {
        // 块作用域
        let b = 2;
        c = b;
        a = 3;
    }

    let b = 3;
    let b = 4;
    // 变量遮蔽
    println!("b:{}", b);
    println!("c:{}", c)
}
