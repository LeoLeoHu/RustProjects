use std::thread;
use std::time::Duration;
use std::sync::mpsc;

// // threads and join
// fn main() {
//     let v = vec![1, 2, 3];
//     let handle = thread::spawn(move || {
//         println!("Here's a vector: {:?}", v);
//         for i in 1..10 {
//             println!("hi number {} from the spawned thread!", i);
//             thread::sleep(Duration::from_millis(1));
//         }
//     });
//     // no more valid
//     // println!("Here's a vector: {:?}", v);
//     for i in 1..5 {
//         println!("hi number {} from the main thread!", i);
//         thread::sleep(Duration::from_millis(1));
//     }
//     handle.join().unwrap();
// }

// // channel
// fn main() {
//     let (tx, rx) = mpsc::channel();
//     thread::spawn(move || {
//         let val = String::from("Hi");
//         tx.send(val).unwrap();
//     });
// 
//     thread::sleep(Duration::from_millis(1));
//     let received = rx.try_recv().unwrap();
//     println!("Got: {}", received);
// }

// channel iterator
fn main() {
    let (tx, rx) = mpsc::channel();
    let tx1 = mpsc::Sender::clone(&tx);

    // sender 1
    let handle = thread::spawn(move || {
        let vals = vec![
        String::from("Hi"),
        String::from("from"),
        String::from("the"),
        String::from("thread"),
        ];
    for val in vals {
        tx.send(val).unwrap();
        thread::sleep(Duration::from_secs(1));
    }
    });
    handle.join().unwrap();

    // sender 2
    thread::spawn(move || {
        let vals = vec![
        String::from("more"),
        String::from("messages"),
        String::from("for"),
        String::from("you"),
        ];
    for val in vals {
        tx1.send(val).unwrap();
        thread::sleep(Duration::from_secs(1));
    }
    });

    // receiver
    for received in rx {
        println!("Got: {}", received);
    }
}
