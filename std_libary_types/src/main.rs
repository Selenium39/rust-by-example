use io::Lines;
use std::fs;
use std::fs::File;
use std::io::Read;
use std::io::Result;
use std::io::Write;
use std::io::{self, BufRead};
use std::path::Path;
use std::process::Command;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;

static THREAD_SIZE: i32 = 5;

fn main() {
    let mut thread_pool = vec![];
    for i in 0..10 {
        // 新建线程
        thread_pool.push(thread::spawn(move || println!("this is thread {}", i)));
    }
    for th in thread_pool {
        // 等待线程结束
        let _ = th.join();
    }

    let (tx, rx): (Sender<i32>, Receiver<i32>) = mpsc::channel();
    for i in 0..THREAD_SIZE {
        // sender端可被复制
        let thread_tx = tx.clone();
        thread::spawn(move || {
            thread_tx.send(i);
        });
    }

    for i in 0..THREAD_SIZE {
        println!("recive:{:?}", rx.recv());
    }

    let path = Path::new(".");
    let display = path.display();
    println!("{}", path.display());
    let new_path = path.join("a").join("b");
    println!("{}", new_path.display());

    let path = Path::new("a.txt");
    let mut file: File = File::open(path).unwrap();
    let mut s = String::new();
    file.read_to_string(&mut s);
    println!("s:{}", s);

    let mut file = File::create("b.txt").unwrap();
    file.write_all("HelloWorld".as_bytes());

    let file = File::open(Path::new("ccc.txt")).unwrap();
    let lines = io::BufReader::new(file).lines();
    for line in lines {
        println!("line:{}", line.unwrap());
    }

    // 进程创建者
    let output = Command::new("rustc")
        .arg("--version")
        .output()
        .unwrap_or_else(|e| panic!("gg"));
    println!("output:{}", output.status);

    let mut child = Command::new("sleep").arg("5").spawn().unwrap();
    //wait 等待child完成
    let _result = child.wait().unwrap();
}
