// 自定义宏
macro_rules! say_hello {
    //不需要参数
    () => {
        // 宏将会展开为代码块里的内容
        println!("hello");
    };
}

macro_rules! create_func {
    // 宏中的参数用$标记
    // ident指示符用于变量名和函数
    ($func_name:ident) => {
        fn $func_name() {
            println!("you called {:?}", stringify!($func_name))
        }
    };
}

macro_rules! print_result {
    //expr指示符用于表达式
    ($result:expr) => {
        println!("this is result:{}", $result)
    };
}

// block
// expr 用于表达式
// ident 用于变量名或函数名
// item
// pat (模式 pattern)
// path
// stmt (语句 statement)
// tt (标记树 token tree)
// ty (类型 type)

fn main() {
    say_hello!();
    create_func!(bar);
    bar();
    print_result!("hello");
}
