pub mod frame;

use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        print_usage();
        process::exit(1);
    }

    let flag = &args[1];
    let path = &args[2];

    match flag.as_str() {
        "-c" => {
            println!("Compressing the image to `probe` at {}", path);
        }
        "-r" => {
            println!("Reading `probe` file");
        }
        _ => {
            eprintln!("Error: Unknown flag '{}'", flag);
            print_usage();
            process::exit(1);
        }
    }
}

fn print_usage() {
    println!("Usage: imgrk [FLAG] [PATH]");
    println!("\nFlags:");
    println!("  -c    Compress the image to `probe` format");
    println!("  -r    Read a `probe` file");
}