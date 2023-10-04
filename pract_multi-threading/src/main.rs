use std::thread;
use std::time::Duration;

fn main() {
    // Spawn the first thread
    let handle1 = thread::spawn(|| {
        for i in 1..10 {
            println!("Thread 1: {}", i);
            thread::sleep(Duration::from_millis(50));
        }
    });

    // Spawn the second thread
    let handle2 = thread::spawn(|| {
        for i in 1..10 {
            println!("Thread 2: {}", i);
            thread::sleep(Duration::from_millis(50));
        }
    });

    // Spawn the third thread
    let handle3 = thread::spawn(|| {
        for i in 1..10 {
            println!("Thread 3: {}", i);
            thread::sleep(Duration::from_millis(50));
        }
    });

    // Wait for all threads to complete
    handle1.join().unwrap();
    handle2.join().unwrap();
    handle3.join().unwrap();
}
