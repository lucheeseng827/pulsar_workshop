use std::sync::{mpsc, Arc, Mutex};
use std::thread;

struct Actor {
    tx: mpsc::Sender<i32>,
}

impl Actor {
    fn new(tx: mpsc::Sender<i32>) -> Self {
        Self { tx }
    }

    fn send(&self, message: i32) {
        self.tx.send(message).expect("Failed to send message");
    }
}

fn main() {
    // Create a channel for message passing
    let (tx, rx) = mpsc::channel();

    // Create an Arc to share the receiver with multiple threads
    let rx = Arc::new(Mutex::new(rx));

    // Spawn two threads that will both receive messages from the channel
    let thread1 = thread::spawn(move || {
        let rx = rx.lock().unwrap();
        for message in rx {
            println!("Thread 1 received message: {}", message);
        }
    });

    let thread2 = thread::spawn(move || {
        let rx = rx.lock().unwrap();
        for message in rx {
            println!("Thread 2 received message: {}", message);
        }
    });

    // Create an actor and send it a message
    let actor = Actor::new(tx);
    actor.send(42);

    // Wait for the threads to finish
    thread1.join().expect("Thread 1 panicked");
    thread2.join().expect("Thread 2 panicked");
}
