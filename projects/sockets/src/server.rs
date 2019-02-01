// extern crate common;
// use common::SOCKET_PATH;
use std::fs;
use std::str;
use std::thread;
use std::io::Read;
use std::os::unix::net::{UnixStream, UnixListener, UnixDatagram};

// fn handle_client(mut stream: UnixStream) {
//     let mut response = String::new();
//     stream.read_to_string(&mut response).unwrap();
//     println!("got response: {:?}", response);
// }

trait Reader {
    fn read(&self, buf: &mut [u8]);
}

impl Reader for UnixDatagram {
    fn read(&self, buf: &mut [u8]) {
        let count = match self.recv(buf) {
            Ok(count) => {
                println!("BUFFER: {:?}", buf);
                println!("READ COUNT: {:?}", count);
                self.send_to(b"34  0 New Msg", "/tmp/rust.sock").expect("send_to function failed");
            }
            Err(e) => {
                println!("Read Err {:?}", e);
            }
        };
    }
}

fn handle_client(dgram: UnixDatagram, buf: &mut Vec<u8>) {
//     // dgram.connect("/tmp/rust.sock").expect("couldn't connect to dgram sock");
//     // let sock = UnixDatagram::bind("/path/to/the/socket").unwrap();
//     // let mut header = vec![0; 6];
    dgram.read(buf);
//     let _m = match dgram.recv(header.as_mut_slice()) {
//         Ok(m) => {
//             // header.resize(60, 0);
//             // let mut mtype = vec![4;6];
//             fs::remove_file("/tmp/rust.sock")
//         }
//         Err(e) => {
//             println!("error on client handle {:?}", e);
//             return
//         }
//     };
//     println!("buf: {:?}", header);
//     let s = str::from_utf8(&header);
//     println!("result: {:?}", s);                    
}

fn read_header(copy_sock: UnixDatagram, header: &mut Vec<u8>) {
    copy_sock.read(header);

    // println!("buf: {:?}", header);
    // let s = str::from_utf8(&header);
    // println!("result: {:?}", s);
    // header.resize(60,0);
}

fn main() {
    fs::remove_file("/tmp/rust.sock");
    println!("Starting server...");
    let sock = match UnixDatagram::bind("/tmp/rust.sock") {
        Ok(sock) => {
            let csock = sock.try_clone().expect("could not copy socket");
            // Create excessive buffer upfront which we will throw away
            // anything greater than this size, we would expect to stream
            let mut header = vec![0; 255];
            read_header(csock, &mut header);

            let mut buf = vec![0; 50];
            handle_client(sock, &mut buf);
        }
        Err(e) => {
            println!("ERR: {:?}", e);
            return
        }
    };
    
    // let listener = UnixListener::bind("/tmp/rust-sock.sock").unwrap();

    // // accept connections and process them, spawning a new thread for each one
    // for stream in listener.incoming() {
    //     match stream {
    //         Ok(stream) => {
    //             /* connection succeeded */
    //             thread::spawn(|| handle_client(stream));
    //         }
    //         Err(err) => {
    //             /* connection failed */
    //             break;
    //         }
    //     }
    // }
}

