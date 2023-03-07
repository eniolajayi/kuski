use std::env;
use kuski::{run_file, run_prompt};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 2 {
        panic!("Usage: kuskit [script]");
    } else if args.len() == 2 {
        run_file(&args[1]);
    } else {
        run_prompt();
    }

    println!("{:?}", &args);
}
