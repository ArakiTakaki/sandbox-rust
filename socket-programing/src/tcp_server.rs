use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::{str, thread};

pub fn server(address: &str) -> Result<(), failure::Error> {
    let listener = TcpListener::bind(address)?; /* [1] */
    loop {
        let (stream, _) = listener.accept()?;

        // threadを起動させることにより複数クライアントの処理を可能にする
        thread::spawn(move || {
            // TODO
            handler(stream).unwrap_or_else(|error| error!("{:?}", error));
        });
    }
}

pub fn handler(mut stream: TcpStream) -> Result<(), failure::Error> {
    debug!("Headline data from {}", stream.peer_addr()?);
    let mut buffer = [0u8; 1024];
    loop {
        let nbytes = stream.read(&mut buffer)?;
        if nbytes == 0 {
            debug!("Connection closed.");
            return Ok(());
        }
        print!("{}", str::from_utf8(&buffer[..nbytes])?);
        stream.write_all(&buffer[..nbytes])?;
    }
}
