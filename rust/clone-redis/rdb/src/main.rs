use std::io::{Read, Write};
use std::net::TcpStream;

#[derive(Debug)]
enum Command {
    Ping,
    Get { key: String },
    Set { key: String, value: String },
}

#[derive(Debug)]
struct Config {
    host: String,
    command: Command,
}

impl Command {
    fn send<W: Write>(&self, dest: &mut W) -> std::io::Result<usize> {
        let bytes_written = match self {
            Command::Ping => dest.write(b"*1\r\n$4\r\nPING\r\n")?,
            Command::Get { key } => {
                let args = &[key.as_bytes()];
                encode_and_write(dest, b"GET", args)?
            }
            Command::Set { key, value } => {
                let args = &[key.as_bytes(), value.as_bytes()];
                encode_and_write(dest, b"SET", args)?
            }
        };

        Ok(bytes_written)
    }
}

fn encode_and_write<W: Write>(
    dest: &mut W,
    command: &[u8],
    args: &[&[u8]],
) -> std::io::Result<usize> {
    let n_args = format!("{}", 1 + args.len());
    let mut bytes_written = 0;

    bytes_written += dest.write(b"*")?;
    bytes_written += dest.write(n_args.as_bytes())?;
    bytes_written += dest.write(b"\r\n")?;

    bytes_written += dest.write(b"$")?;
    bytes_written += dest.write(command.len().to_string().as_bytes())?;
    bytes_written += dest.write(b"\r\n")?;
    bytes_written += dest.write(command)?;
    bytes_written += dest.write(b"\r\n")?;

    for arg in args {
        bytes_written += dest.write(b"$")?;
        bytes_written += dest.write(arg.len().to_string().as_bytes())?;
        bytes_written += dest.write(b"\r\n")?;
        bytes_written += dest.write(arg)?;
        bytes_written += dest.write(b"\r\n")?;
    }

    Ok(bytes_written)
}

impl Config {
    fn from_env() -> Self {
        let args: Vec<_> = std::env::args().into_iter().collect();
        let host = String::from("localhost:6379");

        let pos = 1;
        let command = match args[pos].to_uppercase().as_str() {
            "PING" => Command::Ping,
            "GET" => Command::Get {
                key: args[pos + 1].clone(),
            },
            "SET" => Command::Set {
                key: args[pos + 1].clone(),
                value: args[pos + 2].clone(),
            },
            _ => todo!("{}", args[pos]),
        };

        Config { host, command }
    }
}

fn main() -> std::io::Result<()> {
    let config = Config::from_env();
    // dbg!(&config);

    let mut stream = TcpStream::connect(config.host)?;
    let bytes_sent = config.command.send(&mut stream)?;
    eprintln!("info: {bytes_sent} bytes sent");

    let mut buffer = [0; 1024];
    let _ = stream.read(&mut buffer[..])?;

    let response = std::str::from_utf8(&buffer).unwrap();

    println!("{}", &response);

    Ok(())
}
