#![allow(dead_code, unused_variables)]
use crossbeam::channel;
use std::thread;
use std::time::Duration;

fn expensinve_sum(v: Vec<i32>) -> i32 {
    pause_ms(500);
    println!("Child thread: just about finished");

    v.iter().filter(|&x| x % 2 != 0).map(|&x| x * x).sum()
}

fn pause_ms(ms: u64) {
    thread::sleep(Duration::from_millis(ms));
}

fn main() {
    let my_vector = vec![2, 5, 1, 0, 4, 3];
    let handle = thread::spawn(move || expensinve_sum(my_vector));

    // While the child thread is running, the main thread will also do some work
    for leter in vec!["a", "b", "c", "d", "e", "f"] {
        println!("Main thread: Letter {}", leter);
        pause_ms(200);
    }

    let sum = handle.join().unwrap();
    println!("The child thread's expensive sum is {}", sum);

    let (tx, rx) = channel::unbounded();
    // Cloning a channel makes another variable connected to that end of the channel so that you can
    // send it to another thread.
    let tx2 = tx.clone();
    let handle_a = thread::spawn(move || {
        pause_ms(10);
        tx2.send("Thread A: 1").unwrap();
        pause_ms(200);
        tx2.send("Thread A: 2").unwrap();
    });
    pause_ms(100); // Make sure Thread A has time to get going before we spawn Thread B
    let handle_b = thread::spawn(move || {
        pause_ms(0);
        tx.send("Thread B: 1").unwrap();
        pause_ms(2);
        tx.send("Thread B: 2").unwrap();
    });
    // Using a Receiver channel as an iterator is a convenient way to get values until the channel
    // gets closed.  A Receiver channel is automatically closed once all Sender channels have been
    // closed.  Both our threads automatically close their Sender channels when they exit and the
    // destructors for the channels get automatically called.
    for msg in rx {
        println!("Main thread: Received {}", msg);
    }
    // Join the child threads for good hygiene.
    handle_a.join().unwrap();
    handle_b.join().unwrap();

    let (tx, rx): (channel::Sender<i32>, channel::Receiver<i32>) = channel::bounded(10);
    // Spwan first child thread
    let rx1 = rx.clone();
    let handle1 = thread::spawn(move || loop {
        pause_ms(2);
        match rx1.recv() {
            Ok(val) => println!("Received value in thread 1: {:?}", val),
            Err(_) => break,
        }
    });

    // Spawn second child thread
    let rx2 = rx.clone();
    let handle2 = thread::spawn(move || loop {
        pause_ms(1);
        match rx2.recv() {
            Ok(val) => println!("Received value in thread 2: {}", val),
            Err(_) => break,
        }
    });

    for i in 0..10 {
        println!("Sending value: {}", i);
        tx.send(i).unwrap();
    }

    drop(tx); // Clouse the sending side of the channel

    handle1.join().unwrap();
    handle2.join().unwrap();

    println!("Main thread: Exiting.");
}
