use std::error::Error;

use ntex::{codec::BytesCodec, connect::Connector, util::Buf};

#[ntex::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let connector = Connector::new();

    let stream = connector.connect("127.0.0.1:8080").await?;

    let codec = &BytesCodec;
    for _ in 0..2 {
        stream.send("hello".to_bytes(), codec).await?;
        match stream.recv(codec).await? {
            Some(m) => println!("{}", &String::from_utf8_lossy(&m)),
            None => return Ok(()),
        }
    }

    Ok(())
}
