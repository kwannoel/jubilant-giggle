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

// cargo run --bin test2 > test2.out
// test output is not interleaved for each println! call.
// multiple println! call from various threads can be interleaved.
// can write a property check but im lazy...
//
// Example:
// Thread 81 is running
// Thread 81 is running
// Thread 75 is running
// Thread 77 is running
// --
// Both 81 println are together,
// but 75 and 77 are interleaved.
//
// If we want to use eprintln! to print errors in case of panics, we can
// but it has to be within a single call to println.
// multiple println within same thread are not synchronized.

#[tokio::main]
async fn main() {
    let mut join_handles = vec![];
    for i in 0..100 {
        let hdl = task::spawn(async move {
            println!("Thread {} is running aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa", i);
            println!("Thread {} is running aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa", i);
        });
        join_handles.push(hdl);
    }
    for h in join_handles {
        h.await.unwrap();
    }
}

/*
Thread A running
Thread B running
Thread A hook set
Thread B hook set
panic hook from B triggred with Reason: panicked at 'thread A panic', src/main.rs:33:9
panic hook from B triggred with Reason: panicked at 'called `Result::unwrap()` on an `Err` value: JoinError::Panic(Id(9), ...)', src/main.rs:42:13
*/
