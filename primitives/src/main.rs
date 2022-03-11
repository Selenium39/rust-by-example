use std::fmt;
use std::mem;

fn reverse(pair: (i32, bool)) -> (bool, i32) {
    let (integer, boolean) = pair;
    (boolean, integer)
}

fn transpose(matrix: Matrix) -> Matrix {
    Matrix(matrix.0, matrix.2, matrix.1, matrix.3)
}

#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "( {} , {} )\n", self.0, self.1)?;
        write!(f, "( {} , {} )", self.2, self.3)
    }
}

fn main() {
    let long_tuple = (1, 1.0, true, 'a', ());
    println!("{}", long_tuple.0);
    println!("{}", long_tuple.3);
    println!("{:?}", long_tuple);
    let pair = (1, true);
    println!("{:?}", reverse(pair));
    // 为了和被括号包含的字面量区分，单个元素的元组需要一个,
    println!("{:?}", (1,));
    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
    println!("{:?}", matrix);
    println!("{}", matrix);
    println!("{}", transpose(matrix));

    let arr1: [i32; 5] = [1, 2, 3, 4, 5];
    // 数组里面有100个1
    let arr2: [i32; 100] = [1; 100];
    println!("{}", arr2.len());
    // 数组是在栈中分配的
    println!("{}", mem::size_of_val(&arr2));
    // 切片
    println!("{:?}", &arr1[1..3]);
}
