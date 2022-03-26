fn main() {
    //使用创建的库
    // rustc src/main.rs --extern foo=libfoo.rlib --edition=2018 &&./main
    foo::bar();
}
