// benchmark test for open close, mkdir, rename, chown, chmod, fstat, truncate, 
use std::env;
use std::fs::File;
use std::fs;
use std::path::Path;
use std::time::{Instant, SystemTime};

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    for arg in args  {
        match arg.as_str() {
            "open" => {
                open_test();
            }
            "mkdir" => {
                mkdir_test();
            }
            "rename" => {
                rename_test();
            }
            "chown" => {
                chown_test();
            }
            "chmod" => {
                chmod_test();
            }
            "fstat" => {
                fstat_test();
            }
            "remove" => {
                remove_test();
            }
            "truncate" => {
                truncate_test();
            }
            &_ => {
                ()
            }
        }
    }
}

fn open_test() {
    println!("call open");
    let num = 1000000;
    let file = String::from("foo");
    let f = Path::new(&file);
    File::create(&f).unwrap();
    let now = Instant::now();
    for _i in 1..num {
        let _f = File::open(&file).unwrap();
    }
    let duration: i32 = now.elapsed().as_millis() as i32;
    let ops = num * 1000 / duration;
    println!("open file rate is {}/s", ops);
    fs::remove_file(f);
}

fn mkdir_test() {
    println!("call mkdri");
}

fn rename_test() {
    println!("call rename");
}

fn chown_test() {
    println!("call chown");
}

fn chmod_test() {
    println!("call chmod");
}

fn fstat_test() {
    println!("call fstat");
}

fn remove_test() {
    println!("call remove");
    let num = 100000;
    let file: &str = "foo";
    let f = Path::new(&file);
    let now = Instant::now();
    for _i in 1..num {
        File::create(&f).unwrap();
        fs::remove_file(f);
    }
    let duration: i32 = now.elapsed().as_millis() as i32;
    let ops = num * 1000 / duration;
    println!("open file rate is {}/s", ops);
}

fn truncate_test() {
    println!("call truncate");
}
