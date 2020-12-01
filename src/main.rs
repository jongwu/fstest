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
                fstat_test();
                remove_test();
                read_dir_test();
                chown_test();
                chmod_test();
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
            "read_dir" => {
                read_dir_test();
            }
            &_ => {
                ()
            }
        }
    }
}

fn init_dir() {
    if Path::new(TEST_DIR).exists() {
        clean_up();
    }
    fs::create_dir(TEST_DIR).unwrap();
}

fn init_file(v: &mut Vec<String>) -> Result<File, io::Error> {
    let num = TEST_NUM;
    init_dir();
    
    for i in 1..num {
        let mut basename = String::from(TEST_DIR);
        let filename = basename + &i.to_string();
        let file = Path::new(&filename);
        File::create(&file).unwrap();
        v.push(filename);
    }
    let f = Path::new(TEST_FILE);
    return File::create(&f);
}

fn open_test() {
    println!("call open");
    let mut v: Vec<String> = Vec::new();
    init_file(&mut v);
    if (v.len() == 0) {
        println!("file base is empty");
        panic!();
    }
    let now = Instant::now();
    for file in v.iter() {
        let _f = File::open(&file).unwrap();
    }
    let duration: i32 = now.elapsed().as_millis() as i32;
    let ops = TEST_NUM * 1000 / duration;
    println!("open file rate is {}/s", ops);
    clean_up();
}

fn mkdir_test() {
    println!("call mkdir");
    init_dir();
    let num = TEST_NUM;
    let mut dir: &str = "tmp";
    let now = Instant::now();
    for i in 1..num {
        let basename = String::from(TEST_DIR);
        let d = basename + &i.to_string();
        let path = Path::new(&d);
        fs::create_dir(path).unwrap();
    }
    let duration: i32 = now.elapsed().as_millis() as i32;
    let ops = TEST_NUM * 1000 / duration;
    println!("create directory rate is {}/s", ops);
    clean_up();
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
    println!("rename file rate is {}/s", ops);
    fs::remove_file(TEST_FILE).unwrap();
    clean_up();
}

fn read_dir_test() {
    println!("read directory test");
    let num = TEST_NUM;
    let mut v: Vec<String> = Vec::new();
    init_file(&mut v);

    let now = Instant::now();
    for _i in 1..num {
        fs::read_dir(&Path::new(TEST_DIR)).unwrap();
    }
    let duration: i32 = now.elapsed().as_millis() as i32;
    let ops = num * 1000 / duration;
    println!("read directory rate is {}/s", ops);
    clean_up();
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
}

fn remove_test() {
    println!("call remove");
    let mut v: Vec<String> = Vec::new();
    init_file(&mut v);
    let now = Instant::now();
    for f in v.iter() {
        fs::remove_file(f).unwrap();
    }
    let duration: i32 = now.elapsed().as_millis() as i32;
    let ops = TEST_NUM * 1000 / duration;
    println!("remove file rate is {}/s", ops);
    clean_up();
}

fn truncate_test() {
    println!("call truncate");
}

fn clean_up() {
    let basename = String::from(TEST_DIR);
    if Path::new(TEST_DIR).exists() {
        fs::remove_dir_all(&basename).unwrap();
    }
}
