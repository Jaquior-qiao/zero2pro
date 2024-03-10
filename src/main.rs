//!main.rs
use Zero2pro::run;

#[tokio::main]
async fn main() ->std::io::Result<()>{
    run()?.await
}