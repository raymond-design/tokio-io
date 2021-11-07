use tokio::{io::{AsyncBufReadExt, AsyncWriteExt, BufReader}, net::TcpListener};

#[tokio::main]
async fn main() {
    let listener: TcpListener = TcpListener::bind("localhost:8080").await.unwrap();
    let (mut socket, _addr) = listener.accept().await.unwrap();

    let (read, mut write) = socket.split();
    let mut reader = BufReader::new(read);
    let mut line = String::new();

    loop {
        let byte_count = reader.read_line(&mut line).await.unwrap();
        if byte_count == 0 {
            break;
        }

        write.write_all(&line.as_bytes()).await.unwrap();
        line.clear();
    }
}
 