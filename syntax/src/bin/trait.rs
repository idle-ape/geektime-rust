use core::fmt;
use std::{io::Write, str::FromStr};

use regex::Regex;

struct BufBuilder {
    buf: Vec<u8>,
}

impl BufBuilder {
    pub fn new() -> Self {
        Self {
            buf: Vec::with_capacity(1024),
        }
    }
}

impl fmt::Debug for BufBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", String::from_utf8_lossy(&self.buf))
    }
}

// 可以只实现 trait 的部分方法，其他的都用缺省的实现
impl Write for BufBuilder {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        // 把 buf 添加到 BufBuilder 的尾部
        self.buf.extend_from_slice(buf);
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

fn main() {
    let mut buf = BufBuilder::new();
    buf.write_all(b"Hello world,").unwrap();
    let _ = buf.write(b" By Rust!").unwrap();
    println!("{:?}", buf);
}

pub trait Parse {
    // 关联类型
    type Error;
    fn parse(s: &str) -> Result<Self, Self::Error>
    where
        Self: Sized;
}

impl<T> Parse for T
where
    T: FromStr + Default,
{
    type Error = String;

    fn parse(s: &str) -> Result<Self, Self::Error>
    where
        Self: Sized,
    {
        let re: Regex = Regex::new(r"^[0-9]+(\.[0-9]+)?").unwrap();
        if let Some(captures) = re.captures(s) {
            captures
                .get(0)
                .map_or(Err("failed to capture".to_string()), |s| {
                    s.as_str()
                        .parse()
                        .map_err(|_| "failed to parse captured string".to_string())
                })
        } else {
            Err("failed to parse string".to_string())
        }
    }
}

mod test {
    use super::*;
    #[test]
    fn parse_works() {
        assert_eq!(u32::parse("123abcd"), Ok(123));
        assert_eq!(
            u32::parse("123.45abcd"),
            Err("failed to parse captured string".to_string())
        );
        assert_eq!(f64::parse("1323.567"), Ok(1323.567));
        assert!(f64::parse("abcd").is_err());
    }
}
