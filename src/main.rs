use std::env;
use std::fs::File;
use std::io::{self};
use std::path::Path;

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

fn run_file(filename: impl AsRef<Path>) {
    let file = match File::open(filename) {
        Ok(file) => file,
        Err(error) => {
            println!("An error occurred while opening the file: {:?}", error);
            return;
        }
    };

    let reader = io::BufReader::new(file);
    match io::read_to_string(reader) {
        Ok(s) => run(&s),
        Err(e) => eprintln!("{e}"),
    }
}

fn run(source: &String) {
    println!("{:?}", source);
}

fn run_prompt() {
    println!("kuski repl >> q/quit to exit");
    loop {
        print!("> ");
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        if input.trim() == "quit" || input.trim() == "q" {
            break;
        }
        run(&input);
    }
    println!(">> ");
}
