use std::io::stdin;

use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpStream;
use tokio::sync::mpsc;

const LOCAL_SERVER: &str = "127.0.0.1:8989";

#[tokio::main]
async fn main() ->Result<(), Box<dyn std::error::Error>> {
    println!("tokio client");

    let mut client = TcpStream::connect(LOCAL_SERVER).await?;
    let (tx, mut rx) = mpsc::channel::<String>(3);
    tokio::spawn(async move {
        let (r, mut w) = client.split();
        let mut r = BufReader::new(r);
        let mut line = String::new();
        loop {
            match rx.recv().await {
                Some(msg) => {
                    println!("msg send {}", msg);
                    w.write_all(msg.as_bytes()).await.unwrap();
                }
                None => {

                }
            }
            let bytes_read = r.read_line(&mut line).await.unwrap();
            if bytes_read != 0 {
                println!("recv: {}", line);
            }
        }
    });
    println!("msg:");
    loop {
        let mut buff = String::new();
        stdin().read_line(&mut buff).expect("reading from stdin failed");
        tx.send(buff.clone()).await.unwrap();
        buff.clear();
    }
    Ok(())
}
