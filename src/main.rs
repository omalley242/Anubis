use anubis::cli::process_cli;

#[tokio::main]
async fn main() {
    let result = process_cli().await;
    //Error Handling
    println!("{:?}", result);
}
