#![allow(unused_imports)]
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();

    println!("Waiting for incoming connections...");

    loop {
        let stream = listener.accept().await;

        match stream {
            Ok((mut stream, _)) => {
                println!("accepted new connection");

                tokio::spawn(async move {
                    let mut buf = [0; 1024];
                    loop {
                        let stream_count = stream.read(&mut buf).await.unwrap();
    
                        if stream_count == 0 {
                            break;
                        }
        
                        let buf = b"+PONG\r\n";
                        stream.write(buf).await.unwrap();
                    }
                });

            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
