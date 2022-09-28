use std::thread;
use crossbeam;
use crossbeam::select;
use crossbeam::channel::unbounded;

fn main() {
    let (tx, rx) = unbounded();

    thread::spawn(move || {
        tx.send(42)
            .unwrap()
    });

    select!{
        recv(rx) -> msg => println!("{:?}", msg),
    }
}
