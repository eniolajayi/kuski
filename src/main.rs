use std::env;
use std::fs::File;
use std::io;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 2 {
        panic!("Usage: kuskit [script]");
    } else if args.len() == 2 {
    } else {
        run_prompt();
    }

    println!("{:?}", &args);
}

fn run_file(filename: impl AsRef<Path>) {
    let file = match File::open(filename) {
        Ok(file) => file,
        Err(error) => {
            println!("An error occurred while opening the file: {:?}", error);
            return;
        },
    };

    let reader = io::BufReader::new(file);
    match io::read_to_string(reader) {
        Ok(s) => run(s),
        Err(e) => eprintln!("{e}"),
    }
}

fn run(source: String) {
    todo!();
}

fn run_prompt() {
    todo!();
}
