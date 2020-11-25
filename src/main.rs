// benchmark test for open close, mkdir, rename, chown, chmod, fstat, truncate, 
use std::env;
use std::fs::File;
use std::fs;
use std::io;
use std::path::Path;
use std::time::{Instant};

static TEST_FILE: &str = "/tmp/foo";
static TEST_NUM: i32 = 100000;

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

fn init_file() -> Result<File, io::Error> {
    let num = TEST_NUM;
    for i in 1..num {
        let mut basename = String::from("/tmp/tmp");
        let filename = basename + &i.to_string();
        let file = Path::new(&filename);
        File::create(&file).unwrap();
    }
    let f = Path::new(TEST_FILE);
    File::create(&f)
}

fn open_test() {
    println!("call open");
    let num = TEST_NUM;
    let file = String::from("foo");
    let f = Path::new(&file);
    File::create(&f).unwrap();
    init_file();
    let now = Instant::now();
    for i in 1..num {
//        let mut basename = String::from("/tmp/tmp");
//        let filename = basename + &i.to_string();
//        let _f = File::open(&filename).unwrap();
        let _f = File::open(&file).unwrap();
    }
    let duration: i32 = now.elapsed().as_millis() as i32;
    let ops = num * 1000 / duration;
    println!("open file rate is {}/s", ops);
    fs::remove_file(f).unwrap();
}

fn mkdir_test() {
    println!("call mkdri");
}

fn rename_test() {
    println!("call rename");
    let tmp_file: &str = "/tmp/tmp";
    let num = 100000;
    init_file();
    let now = Instant::now();
    for _i in 1..num {
        let _f = fs::rename(TEST_FILE, tmp_file).unwrap();
        let _f = fs::rename(tmp_file, TEST_FILE).unwrap();
    }
    let duration: i32 = now.elapsed().as_millis() as i32;
    let ops = num * 2000 / duration;
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
    let f = init_file().unwrap();
    let now = Instant::now();
    for i in 1..num {
        let basename = String::from("/tmp/tmp");
        let filename = basename + &i.to_string();
        let home = Path::new("/home");
        let f = Path::new(&filename);
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
    let num = 100000;
    let file: &str = "foo";
    let f = Path::new(&file);
    let now = Instant::now();
    for _i in 1..num {
        File::create(&f).unwrap();
        fs::remove_file(f).unwrap();
    }
    let duration: i32 = now.elapsed().as_millis() as i32;
    let ops = num * 1000 / duration;
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
