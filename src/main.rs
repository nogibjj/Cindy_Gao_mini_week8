// fn main() {
//     println!("Hello, world!");
// }

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version = "1.0", about = "a to the power of b")]
struct Args {
    a: i64,
    b: i64,
}

fn exponent(a: i64, b: i64) -> i64 {
    if b < 0 {
        panic!("Exponent must be non-negative!");
    }

    let mut result = 1;
    for _ in 0..b {
        result *= a; // Multiply result by a, b times
    }
    result
}

fn main() {
    let args = Args::parse();

    let output = exponent(args.a, args.b);
    println!("The result of a to the power of b is: {}", output)
}
