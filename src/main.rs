//!main.rs
use std::net::TcpListener;
use Zero2pro::startup::run;

#[tokio::main]
async fn main() ->std::io::Result<()>{
    let listener = TcpListener::bind("127.0.0.1:0")
        .expect("Fail to build a listener");
    run(listener)?.await
}