use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: cat <file>");
        println!("Example: cat hello.txt");
    } else {
        match fs::read_to_string(&args[1]) {
            Ok(content) => {
                let mut dashes = String::new();
                for _ in 0..args[1].len()+2 {
                    dashes += "-";
                }
                println!("{}\n|{}|\n{}\n{}", dashes, &args[1], dashes, content);
            }
            Err(_) => {println!("Could not read file '{}'.", args[1])}
        }
    }
}
