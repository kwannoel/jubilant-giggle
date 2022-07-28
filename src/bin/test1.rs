use std::panic;
use std::thread;
use std::time::Duration;
use tokio::task;

// Test 1:
// Panic hook is unsuitable since it is global resource,
// not local to thread.
//
// Test Steps:
// Thread A sets panic hook, sleeps, then panics.
// Thread B sets panic hook
//
// A -------> set hook A
// |
// |    B --> set hook B
// |
// ----------> panic
//
// We should see hook B triggered,
// when A panics,
// since panic hook are global resource

#[tokio::main]
async fn main() {
    let a = task::spawn(async move {
        println!("Thread A running");
        panic::set_hook(Box::new(|e| {
            eprintln!("panic hook from A triggered with Reason: {}", e)
        }));
        println!("Thread A hook set");
        thread::sleep(Duration::from_millis(10000));
        panic!("thread A panic");
    });
    let b = task::spawn(async move {
        println!("Thread B running");
        panic::set_hook(Box::new(|e| {
            eprintln!("panic hook from B triggred with Reason: {}", e)
        }));
        println!("Thread B hook set");
    });
    a.await.unwrap();
    b.await.unwrap();
}

/*
Thread A running
Thread B running
Thread A hook set
Thread B hook set
panic hook from B triggred with Reason: panicked at 'thread A panic', src/main.rs:33:9
panic hook from B triggred with Reason: panicked at 'called `Result::unwrap()` on an `Err` value: JoinError::Panic(Id(9), ...)', src/main.rs:42:13
*/
