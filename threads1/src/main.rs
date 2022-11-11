use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thred!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    handle.join().expect("waiting for the spawned thread to finish");

    for i in 1..5 {
        println!("hi number {} from the main thred!", i);
        thread::sleep(Duration::from_millis(1));
    }

    // handle.join().expect("waiting for the spawned thread to finish");

    let v = vec![1,2,3];
    let handle2 = thread::spawn(move || {
        println!("printing vector: {:?}", v);
    });

    handle2.join().expect("waiting for the spawned thread to finish");

}
