use std::sync::mpsc;
use std::thread;

fn main() {
    let x = 5;
    let add = |y| x + y;
    let result = add(2);

    println!("{result}"); // Output: 7

    let nums = vec![1, 2, 3, 4, 5];
    let sum = nums.iter().fold(0, |acc, x| acc + x);
    println!("Sum: {}", sum);

    let (tx, rx) = mpsc::channel();

    let child_thread = thread::spawn(move || {
        // send a message to the main thread
        tx.send("Hello from the child thread!").unwrap();
    });

    // receive the message sent by the child thread
    let message = rx.recv().unwrap();
    println!("Message: {}", message);

    // The join() method is called on the child_thread to wait for it to finish executing before continuing with the main thread
    child_thread.join().unwrap();
}
