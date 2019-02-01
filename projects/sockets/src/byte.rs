use std::str;
use std::mem;
use std::io::Read;

fn main() {
    let n = std::mem::size_of_val("{Event: Syn}");
    println!("n {:?}", n);
    let t = n.to_ne_bytes();
    println!("t {:?}", t);
    let q = usize::from_ne_bytes(t);
    println!("q {:?}", q);
    let s = str::from_utf8(&t).unwrap();
    println!("s {:?}", s);
    // let mut p = [32, 1]
    // println!("t {:?}", t.bytes().from_ne_bytes());
    // let msg = b"{Event: Syn}";
    // println!("len {:?}", msg.len());
}