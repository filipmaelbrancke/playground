use rdb::redis_encoding;
use std::io::{BufRead, BufReader, Write};
use std::net::TcpStream;

/*
Will generate something like::

| TcpStream {
|     addr: 127.0.0.1:64674,
|     peer: 127.0.0.1:6379,
|     fd: 3,
| }
> 14 bytes transmitted
| *1
| $4
| PING
|
< 7 bytes transmitted
| +PONG
|
*/
fn main() -> std::io::Result<()> {
    let host = "127.0.0.1";
    let port = 6379;
    let command = vec!["PING"];

    let mut server = TcpStream::connect(format!("{host}:{port}"))?;
    server_info(&server);

    let req = redis_encoding(command);

    if let Ok(bytes_tx) = server.write(&req) {
        tx_info('>', bytes_tx, &req);
    };

    let mut reader = BufReader::new(&server);
    let mut buffer = Vec::new();
    if let Ok(bytes_tx) = reader.read_until(b'\n', &mut buffer) {
        tx_info('<', bytes_tx, &buffer);
    };

    Ok(())
}

fn tx_info(direction: char, bytes_tx: usize, msg: &[u8]) {
    println!("{direction} {bytes_tx} bytes transmitted");

    let text = String::from_utf8_lossy(msg);
    for line in text.split("\r\n") {
        println!("| {line}")
    }
}

fn server_info(server: &TcpStream) {
    let info = format!("{server:#?}");
    for line in info.lines() {
        println!("| {line}")
    }
}
