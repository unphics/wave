const LOCAL_SERVER: &str = "127.0.0.1:8989";
use std::io::BufReader;

use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    println!("tokio server");

    let listener = TcpListener::bind(LOCAL_SERVER).await?;
    loop {
        let (mut sock, addr) = listener.accept().await?;
        println!("{} connected", addr);
        tokio::spawn(async move {
            let (reader, mut writer) = sock.split();
            let mut reader = BufReader::new(reader);
            -- TODO LAST
        });
    }

    Ok(())
}
