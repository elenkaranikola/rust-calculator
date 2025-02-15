use clap::Parser;
use meval::eval_str;


#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    expression: String,
}

fn main() {

    let args = Args::parse();

    match eval_str(&args.expression) {
        Ok(result) => println!("Result: {}", result),
        Err(err) => eprintln!("Error evaluating expression: {}", err),
    }
}
