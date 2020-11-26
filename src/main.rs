// benchmark test for open close, mkdir, rename, chown, chmod, fstat, truncate, 
use std::env;
use std::fs::File;
use std::fs;
use std::io;
use std::path::Path;
use std::time::{Instant};

static TEST_FILE: &str = "/tmp/foo";
static TEST_NUM: i32 = 100000;
static TEST_DIR: &str = "/tmp/fstest/";

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    for arg in args  {
        match arg.as_str() {
            "-a" => {
                open_test();
                mkdir_test();
                rename_test();
                chown_test();
                chmod_test();
                fstat_test();
                remove_test();
                truncate_test();
            }
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

fn init_file(v: &mut Vec<String>) -> Result<File, io::Error> {
    let num = TEST_NUM;
    if !Path::new(TEST_DIR).exists() {
        fs::create_dir(TEST_DIR).unwrap();
    }
    for i in 1..num {
        let mut basename = String::from(TEST_DIR);
        let filename = basename + &i.to_string();
        let file = Path::new(&filename);
        File::create(&file).unwrap();
        v.push(filename);
    }
    let f = Path::new(TEST_FILE);
    File::create(&f)
}

fn open_test() {
    println!("call open");
    let mut v: Vec<String> = Vec::new();
    init_file(&mut v);
    let now = Instant::now();
    if (v.len() == 0) {
        println!("file base is empty");
        panic!();
    }
    for file in v.iter() {
        let _f = File::open(&file).unwrap();
    }
    let duration: i32 = now.elapsed().as_millis() as i32;
    let ops = TEST_NUM * 1000 / duration;
    println!("open file rate is {}/s", ops);
}

fn mkdir_test() {
    println!("call mkdri");
}

fn rename_test() {
    println!("call rename");
    let tmp_file: &str = &(String::from(TEST_DIR) + "tmp");
    let mut v: Vec<String> = Vec::new();
    init_file(&mut v);
    let now = Instant::now();
    for file in v.iter(){
        let _f = fs::rename(file, tmp_file).unwrap();
    }
    let duration: i32 = now.elapsed().as_millis() as i32;
    let ops = TEST_NUM * 1000 / duration;
    println!("open file rate is {}/s", ops);
    fs::remove_file(TEST_FILE).unwrap();
}

fn chown_test() {
    println!("call chown");
}

fn chmod_test() {
    println!("call chmod");
}

fn fstat_test() {
    println!("call fstat");
    let num = TEST_NUM;
    let mut v: Vec<String> = Vec::new();
    let f = init_file(&mut v).unwrap();
    let mut path: Vec<&Path> = Vec::new();
    for file in v.iter() {
        let f = Path::new(file);
        path.push(f);
    }
    let now = Instant::now();
    for f in path.iter() {
        f.metadata().unwrap();
    }
    let duration: i32 = now.elapsed().as_millis() as i32;
    let ops = num * 1000 / duration;
    println!("read file metadata rate is {}/s", ops);
    clean_up();
//    println!("metadata of file is {:?}", metadata);
}

fn remove_test() {
    println!("call remove");
    let mut v: Vec<String> = Vec::new();
    init_file(&mut v);
    let now = Instant::now();
    for f in v.iter() {
//        File::create(&f).unwrap();
        fs::remove_file(f).unwrap();
    }
    let duration: i32 = now.elapsed().as_millis() as i32;
    let ops = TEST_NUM * 1000 / duration;
    println!("open file rate is {}/s", ops);
}

fn truncate_test() {
    println!("call truncate");
}

fn clean_up() {
    let num = TEST_NUM;
    for i in 1..num {
        let basename = String::from("/tmp/tmp");
        let filename = basename + &i.to_string();
        fs::remove_file(&filename).unwrap();
    }
    fs::remove_file(TEST_FILE).unwrap();
}
