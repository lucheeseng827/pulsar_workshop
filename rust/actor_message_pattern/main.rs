use std::sync::{mpsc, Arc, Mutex};
use std::thread;

struct Actor {
    rx: mpsc::Receiver<i32>,
}

impl Actor {
    fn new(rx: mpsc::Receiver<i32>) -> Self {
        Self { rx }
    }

    fn start(self) {
        thread::spawn(move || {
            for message in self.rx {
                println!("Actor received message: {}", message);
            }
        });
    }
}

fn main() {
    // Create a channel for message passing
    let (tx, rx) = mpsc::channel();

    // Create an Arc to share the receiver with the actor
    let rx = Arc::new(Mutex::new(rx));

    // Create an actor that will consume messages from the channel
    let actor = Actor::new(rx);
    actor.start();

    // Publish some messages to the channel
    for i in 0..10 {
        tx.send(i).expect("Failed to send message");
    }
}
