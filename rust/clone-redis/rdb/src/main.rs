use std::io::{Read, Write};
use std::net::TcpStream;

#[derive(Debug)]
enum Command {
    Ping,
}

#[derive(Debug)]
struct Config {
    host: String,
    command: Command,
}

impl Command {
    fn send<W: Write>(&self, dest: &mut W) -> std::io::Result<usize> {
        let written = match self {
            Command::Ping => dest.write(b"*1\r\n$4\r\nPING\r\n")?,
        };

        Ok(written)
    }
}

impl Config {
    fn from_env() -> Self {
        let args: Vec<_> = std::env::args().into_iter().collect();
        let host = String::from("localhost:6379");

        let pos = 1;
        let command = match args[pos].to_uppercase().as_str() {
            "PING" => Command::Ping,
            _ => todo!("{}", args[pos]),
        };

        Config { host, command }
    }
}

fn main() -> std::io::Result<()> {
    let config = Config::from_env();
    // dbg!(&config);

    let mut stream = TcpStream::connect(config.host)?;
    let command = config.command;
    command.send(&mut stream)?;

    let mut buffer = [0; 1024];
    let _ = stream.read(&mut buffer[..])?;

    let response = std::str::from_utf8(&buffer).unwrap();

    println!("{}", &response);

    Ok(())
}
