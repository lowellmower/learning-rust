extern crate tokio;

use tokio::io::*;
use tokio::net::*;
use tokio::prelude::*;

use std::fmt;
use std::fs::remove_file;

#[derive(Debug)]
struct Environment {
    mailbox: Mailbox,
    listener: UnixListener,
}

#[derive(Debug)]
struct Event {
    msg: Message
}

impl Event {
    pub fn new(message: Message) -> Self {
        Event { msg: message }
    }
}

impl fmt::Display for Event {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Event({})", self.msg)
    }
}

#[derive(Debug)]
enum Message {
    Syn,
    Ack,
}

impl fmt::Display for Message {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Message::Syn => write!(f, "SYN"),
            Message::Ack => write!(f, "ACK"),
        }
    }
}

#[derive(Debug, Clone)]
enum Mailbox {
    InternalTransport {
        addr: String
    },
    ExtneralTransport {
        addr: String
    },
    DataStore {
        addr: String
    },
}

impl Actor for Mailbox {
    type Mailbox = Self;
    fn send(&mut self) {
        println!("Calling 'send()' for Mailbox type {}", self);
    }
}

impl fmt::Display for Mailbox {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Mailbox::InternalTransport{addr: _} => write!(f, "InternalTransport"),
            Mailbox::ExtneralTransport{addr: _} => write!(f, "ExternalTransport"),
            Mailbox::DataStore{addr: _} => write!(f, "DataStore")
        }
    }
}

trait Actor {
    type Mailbox;
    // type Error;
    fn send(&mut self);
}

fn main() -> Result<()> {
    let _rm = remove_file("/tmp/example.sock");
    let mut env = Environment {
                    mailbox: Mailbox::DataStore {addr: "/tmp/example.db".to_string()},
                    listener: UnixListener::bind("/tmp/example.sock").unwrap(),
            };
    
    // clone this only for the purpose of example to show how the send()
    // method from Actor trait might be used as a general implementation
    let mut env_mb = env.mailbox.clone();

    let done = env.listener.incoming()
        .map_err(|e| println!("failed to accept socket; error = {:?}", e))
        .for_each(move |socket| {
            // All this is currently doing is "echoing" the bytes read
            // back on to the same socket. This is a trivial example.
            let (reader, writer) = socket.split();
            let amt = tokio::io::copy(reader, writer);
            let msg = amt.then(move |result| {
                match result {
                    Ok((amt, _, _)) => {
                        println!("wrote {} bytes immediately", amt);
                    },
                    Err(e) => println!("error: {}", e),
                }

                Ok(())
            });
            env_mb.send();
            tokio::spawn(msg)
        });
    tokio::run(done);
    Ok(())
}
