use anubis::cli::process_cli;

fn main() {
    let result = process_cli();
    //Error Handling
    println!("{:?}", result);
}
