struct MyDrop;

struct Person {
    name: String,
    age: u8,
}

#[derive(Debug)]
struct Book {
    // &'static 分配在只读内存区的引用
    author: &'static str,
    title: &'static str,
    year: u32,
}

fn create_box() {
    let box1 = Box::new(3i32);
}

impl Drop for MyDrop {
    fn drop(&mut self) {
        println!("析构函数被调用")
    }
}

fn changeBook(book: &mut Book) {
    book.year = 11
}

fn readBook(book: &Book) {
    println!("{:?}", book)
}

// 显示的标注生命周期
fn print_ref<'a, 'b>(a: &'a i32, b: &'b i32) {
    println!("{},{}", a, b)
}

fn main() {
    for _ in 0..100 {
        // 不需要手动释放内存 RAII
        create_box()
    }
    let my_drop = MyDrop;

    let person = Person {
        name: String::from("Selenium39"),
        age: 24,
    };
    // name的所有权被转移，但是age的所有权未转移
    let Person { name, ref age } = person;
    println!("age:{}", person.age);
    // 因为name的所有权被转移，所以无法被使用
    // println!("name:{}", person.name);

    // 不可变引用
    let book = Book {
        title: "rust中文",
        author: "selenium39",
        year: 10,
    };

    // 可变引用
    let mut book1 = book;

    // 不可变的借用不可变对象
    // changeBook(&book);
    // readBook(&book);
    // 可变的借用可变对象
    changeBook(&mut book1);

    let c = 'C';
    // 左边的ref等价于右边的&
    let ref_c1 = &c;
    let ref ref_c2 = c;

    let (a, b) = (1, 2);
    print_ref(&a, &b);
}
