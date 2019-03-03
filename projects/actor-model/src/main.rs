extern crate actix;
extern crate futures;

use futures::Future;
use actix::prelude::*;

// #[derive(Debug)]
struct MsgActor {
    count: usize,
}

impl Actor for MsgActor {
    type Context = Context<Self>;
}

struct Ping(usize);

impl Message for Ping {
    type Result = usize;
}

impl Handler<Ping> for MsgActor {
    type Result = usize;

    fn handle(&mut self, msg: Ping, _ctx: &mut Context<Self>) -> Self::Result {
        self.count += msg.0;
        self.count
    }
}

fn main() {
    let actor_system = System::new("demo");

    // start the actor to get address
    let ma_rx = MsgActor{count: 5}.start();
    
    // send msg and get future for result
    let ma_future = ma_rx.send(Ping(3));

    Arbiter::spawn(
        ma_future.map(|ma_future| {
            System::current().stop();
            println!("RESULT: {}", ma_future);
        })
        .map_err(|_|()));

    actor_system.run();
}






