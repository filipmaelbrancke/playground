use std::io::{Read, Write};
use std::net::TcpStream;

fn main() -> std::io::Result<()> {
    let mut stream = TcpStream::connect("localhost:6379")?;
    stream.write_all(b"*1\r\n$4\r\nPING\r\n")?;

    let mut buffer = [0; 1024];
    let _ = stream.read(&mut buffer[..])?;

    let response = std::str::from_utf8(&buffer).unwrap();

    println!("{}", &response);

    Ok(())
}
