use std::io;

use ntex::{
    codec::BytesCodec,
    fn_service,
    io::{Filter, Framed, Io},
    server::Server,
    util::Either,
};

#[ntex::main]
async fn main() -> io::Result<()> {
    let addr = ("0.0.0.0", 8080);
    Server::build()
        .bind("echo", addr, |_| fn_service(echo))?
        .run()
        .await
}

async fn echo<F: Filter>(s: Io<F>) -> io::Result<()> {
    let framd = Framed::new(s, BytesCodec);
    loop {
        match framd.recv().await.to()? {
            Some(m) => framd.send(m.freeze()).await.to()?,
            None => return Ok(()),
        }
    }
}

trait To<T> {
    fn to(self) -> io::Result<T>;
}

impl<T> To<T> for Result<T, Either<io::Error, io::Error>> {
    fn to(self) -> io::Result<T> {
        match self {
            Ok(o) => Ok(o),
            Err(e) => Err(match e {
                Either::Left(left) => left,
                Either::Right(right) => right,
            }),
        }
    }
}
