// use std::io::{BufWriter, Write};
// use std::net::TcpStream;

// #[derive(Debug)]
// struct MyWriter<W> {
//     writer: W,
// }

// impl MyWriter<BufWriter<TcpStream>> {
//     pub fn new(addr: &str) -> Self {
//         let stream = TcpStream::connect(addr).unwrap();
//         Self {
//             writer: BufWriter::new(stream),
//         }
//     }

//     pub fn write(&mut self, buf: &str) -> std::io::Result<()> {
//         self.writer.write_all(buf.as_bytes())
//     }
// }

// fn main() {
//     let mut writer = MyWriter::new("127.0.0.1:8080");
//     let _ = writer.write("hello world!");
// }

use rand::RngExt;

fn main() {
    let mut data: Vec<Vec<u8>> = Vec::new();

    for _i in 0..5 {
        let mut num: Vec<u8> = Vec::new();
        for _j in 0..16 {
            let rand_num: u8 = rand::rng().random();
            num.push(rand_num);
        }
        println!("num is : {:?},num.as_ptr(): {:?}", num, num.as_ptr());
        data.push(num);
    }

    println!(
        "data is: {:?}",
        data.iter().map(|v| v.as_ptr()).collect::<Vec<_>>()
    );
}
