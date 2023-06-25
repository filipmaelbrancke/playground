// Redis Bulk Strings = https://redis.io/docs/reference/protocol-spec/#resp-bulk-strings
// check the balance between using the type system to guard against sending invalid data vs the ease of using Vec<u8>

struct BulkString(Vec<u8>);

trait ToBulkString {
    fn to_bulk_string(&self) -> BulkString;
}

impl ToBulkString for &str {
    fn to_bulk_string(&self) -> BulkString {
        let mut out = Vec::with_capacity(self.len());
        let len = self.len().to_string();
        out.push(b'$');
        out.extend_from_slice(len.as_bytes());
        out.extend_from_slice(b"\r\n");
        out.extend_from_slice(self.as_ref());
        out.extend_from_slice(b"\r\n");
        BulkString(out)
    }
}

impl ToBulkString for &[u8] {
    fn to_bulk_string(&self) -> BulkString {
        let mut out = Vec::with_capacity(self.len());
        let len = self.len().to_string();
        out.push(b'$');
        out.extend_from_slice(len.as_bytes());
        out.extend_from_slice(b"\\r\\n");
        out.extend_from_slice(self.as_ref());
        out.extend_from_slice(b"\\r\\n");
        BulkString(out)
    }
}

impl std::ops::Deref for BulkString {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub fn redis_encoding(strings: Vec<&str>) -> Vec<u8> {
    let len = format!("{}", strings.len());
    let mut out = vec![];
    out.push(b'*');
    out.extend_from_slice(len.as_bytes());
    out.extend_from_slice(b"\r\n");
    for s in strings {
        let bulk = s.to_bulk_string();
        out.extend_from_slice(&bulk);
    }
    out
}

// ------------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use crate::{redis_encoding, ToBulkString};

    #[test]
    fn test_bulk_string() {
        let msg = "rdb, a redis clone 💖";
        let bulk_msg = msg.to_bulk_string();

        let bulk = String::from_utf8_lossy(&bulk_msg);

        assert_eq!("$23\r\nrdb, a redis clone 💖\r\n", bulk);
    }

    #[test]
    fn test_redis_encoding() {
        let msg_parts = vec!["rdb", "a redis clone"];
        let bulk_raw = redis_encoding(msg_parts);

        let bulk = String::from_utf8_lossy(&bulk_raw);

        assert_eq!("*2\r\n$3\r\nrdb\r\n$13\r\na redis clone\r\n", bulk);
    }

    #[test]
    fn test_redis_set_command() {
        let msg_parts = vec!["SET", "welcome", "Hello, rdb"];
        let bulk_raw = redis_encoding(msg_parts);

        let bulk = String::from_utf8_lossy(&bulk_raw);

        assert_eq!(
            "*3\r\n$3\r\nSET\r\n$7\r\nwelcome\r\n$10\r\nHello, rdb\r\n",
            bulk
        );
    }
}
