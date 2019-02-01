#![feature(rustc_private)]

use std::fs;
use std::str;
use std::mem;
// use std::thread;
use std::io::{Read, Write};
use std::os::unix::net::{UnixStream, UnixListener};
// use std::net::Shutdown;

use byteorder::*;
extern crate byteorder;

trait Reader {
    fn reader(&mut self) -> Vec<u8>;
}

trait Writer {
    fn writer(&mut self, String);
}

// impl Reader for UnixStream {
//     fn reader(&mut self) -> Vec<u8> {
//         // clone so we may have two mutable borrows, one here
//         // and one where self.bytes() is called
//         let mut copy = self.try_clone().expect("fail to clone");
//         let mut header = copy.take(4);
//         // let mut sbuffer = vec![0; 4];
//         let mut abuffer = [0; 4];

//         header.read(&mut abuffer);
//         println!("read header {:?}", header);
//         println!("read buffer {:?}", abuffer);

//         // abuffer.copy_from_slice(&sbuffer[0..4]);
//         let h = u32::from_ne_bytes(abuffer);
//         // let h = str::from_utf8(&mut buffer).unwrap();
//         println!("Read from head: {:?}", h);

//         // let body_size = h.parse::<u8>().unwrap();
//         println!("As num: {:?}", h);

//         let take_bound = h as u64;
//         let end = h as usize;
//         let mut body_buffer = vec![0; end];
//         let mut taker = self.take(take_bound);

//         taker.read(&mut body_buffer);
//         return body_buffer;
//     }
// }
impl Reader for UnixStream {
    fn reader(&mut self) -> Vec<u8> {
        let copy = self.try_clone().expect("fail to clone");
        let mut header = copy.take(10);
        let mut buffer = [0; 10];

        header.read(&mut buffer);
        let h = str::from_utf8(&buffer).unwrap().trim_end();
        println!("Header: {:?}", h);

        let body_size = h.parse::<u64>().unwrap();
        println!("As num: {:?}", body_size);

        let end = body_size as usize;
        let mut body_buffer = vec![0; end];
        let mut body = self.take(body_size);

        body.read(&mut body_buffer);
        return body_buffer;
    }
}

impl Writer for UnixStream {
    fn writer(&mut self, data: String){
        // let mut header = vec![0; 4];
        // let body = data.as_bytes();
        // // let bsize = std::mem::size_of_val(&data).to_be_bytes();
        // let mut bsize = body.len().to_ne_bytes();
        
        // header.splice(..bsize.len(), bsize.iter().cloned());
        // header.extend(body.to_vec());

        // println!("Header {:?}", header);
        // println!("Body {:?}", body);
        
        self.write_all(data.as_bytes());
   }
}

fn handle_client(mut stream: UnixStream) {
    let v = stream.reader();
    // stream.writer("{Event: Ack}".to_string());
    let s = str::from_utf8(v.as_slice());
    println!("Body {:?}", s);
    reply(stream);
    // stream.write_all(b"12        {Event: Ack}");
}

fn reply(mut stream: UnixStream) {
    stream.writer("{Event: Ack}".to_string());
}

fn main() {
    fs::remove_file("/tmp/rust.sock");
    println!("Starting server...");
    let listener = UnixListener::bind("/tmp/rust.sock").unwrap();
    loop {
        match listener.accept() {
            Ok((stream, _)) => {
                handle_client(stream);
            }
            Err(e) => {
                println!("Error handling connection {:?}", e);
                break;
            }
        }
    }
}
