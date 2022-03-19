fn main() {
    let x = 5u32;

    let y = {
        let x_area = x * x;
        let x_cube = x_area * x;
        x_area * x_cube * x
    };

    let z = {
        // 由于这里有;所以返回值是()
        x * 2;
    };

    println!("{:?}", x);
    println!("{:?}", y);
    println!("{:?}", z);
}
