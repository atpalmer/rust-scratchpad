use chrono::UTC;
use std::sync::Arc;
use std::thread::JoinHandle;

struct Data {
    message: String,
    time: chrono::DateTime<chrono::UTC>,
}

fn spawn(count: i32, data_ref: &Arc<Data>) -> JoinHandle<()> {
    let data = data_ref.clone();
    return std::thread::spawn(move || {
        println!("Thread {} (at {}) relaying: \"{}\" from time {:#?}", count, UTC::now(), data.message, data.time);
    });
}

pub fn main() {
    let data = Data {
        message: String::from("Hello, world"),
        time: UTC::now(),
    };
    let data_ref = Arc::new(data);

    let threads = vec![
        spawn(1, &data_ref),
        spawn(2, &data_ref),
        spawn(3, &data_ref),
        spawn(4, &data_ref),
        spawn(5, &data_ref),
        spawn(6, &data_ref),
        spawn(7, &data_ref),
        spawn(8, &data_ref),
    ];

    for thread in threads {
        let result = thread.join();
        match result {
            Ok(_) => (),
            Err(e) => { eprintln!("Error: {:#?}", e); }
        }
    }
}
