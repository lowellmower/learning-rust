extern crate clap;

use clap::{App, Arg};
use std::process::{Command, Stdio};

fn write_socket() {
    let mut stream = UnixStream::connect("/tmp/rust-sock.sock").unwrap();
    stream.write_all(b"hello world").unwrap();
}

fn main() {
    let app = App::new("Example CLI")
                    .version("0.1.0")
                    .author("Lowell Mower <lowell.mower@gmail.com>")
                    .about("A small CLI for learning")
                    .arg(Arg::with_name("eko")
                        .short("e")
                        .long("eko")
                        .value_name("eko")
                        .help("Echos arguments")
                        .takes_value(true)
                        .required(false))
                    .arg(Arg::with_name("list")
                        .short("l")
                        .long("list")
                        .value_name("list")
                        .help("Lists the contents of a file or dir")
                        .takes_value(true)
                        .required(false))
                    .arg(Arg::with_name("net")
                        .short("-n")
                        .long("net")
                        .value_name("net")
                        .help("Netcat <host> <port>")
                        .multiple(true)
                        .required(false))
                    .get_matches();


    if app.is_present("eko") {
        let input = app.value_of("eko").unwrap();
        println!("Input: {:?}", input);
    }

    if app.is_present("list") {
        let loc = app.value_of("list").unwrap();
        let output = Command::new("ls")
                .arg("-la")
                .arg(loc)
                .output()
                .expect("failed to execute process");
    
        println!("List: {:?}", String::from_utf8_lossy(&output.stdout));
    }

    if app.is_present("net") {
        let net_vals: Vec<&str> = app.values_of("net").unwrap().collect();
        println!("Net: {:?}", net_vals);
        let nc = Command::new("nc")
                    .arg("-l")
                    .arg(net_vals[0])
                    .arg(net_vals[1])
                    .stdout(Stdio::piped())
                    .stderr(Stdio::piped())
                    .spawn()
                    .expect("failed to execute process");
        let id = nc.id();
        println!("Id: {:?}", id);
        let output = nc.wait_with_output().unwrap();
        println!("Out: {:?}", String::from_utf8_lossy(&output.stdout));
    }
}
