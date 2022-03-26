// 将一个完整的路径绑定到一个新名字，方便访问
use myMod::public_func as pfnc;
mod foo;

mod myMod {
    pub struct pub_struct {
        pub public_item: i32, // 结构体中的public 属性
        private_item: i32,    // 结构体中的private属性
    }

    // 默认是private
    fn private_func() {
        println!("private_func")
    }

    // 使用pub改变其可见性
    pub fn public_func() {
        println!("public func")
    }

    pub fn access_func() {
        println!("access_func");
        private_func()
    }
    pub mod myMod1 {
        fn public_func() {
            println!("public func1 myMod1")
        }

        pub fn calc() {
            // self为当前作用域
            self::public_func();
            // super为父作用域
            super::public_func();
        }
    }
}

fn main() {
    myMod::public_func();
    myMod::access_func();
    pfnc();
    myMod::myMod1::calc();
    // 调用不同文件mod的方法
    foo::bar()
}
