// benchmark test for open close, mkdir, rename, chown, chmod, fstat, truncate, 
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    for arg in args  {
        match arg.as_str() {
            "open" => {
                open_test();
            }
            &_ => {
                println!("not recgenized");
            }
        }
    }
}

fn open_test() {
    println!("call open");
}
