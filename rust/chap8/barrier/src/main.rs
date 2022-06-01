use std::sync::mpsc::{channel, Sender};

fn main() {
    let mut v = Vec::new();

    let (tx, rx) = channel::<Sender<()>>();

    let barrier = move || {
        let x = rx.recv().unwrap();
        let y = rx.recv().unwrap();
        let z = rx.recv().unwrap();
        println!("send!");
        x.send(()).unwrap();
        y.send(()).unwrap();
        z.send(()).unwrap();
    };
    let t = std::thread::spawn(barrier);
    v.push(t);

    for _ in 0..3 {
        let tx_c = tx.clone();
        let node = move || {
            let (tx0, rx0) = channel();
            tx_c.send(tx0).unwrap();
            rx0.recv().unwrap();
            println!("received!");
        };
        let t = std::thread::spawn(node);
        v.push(t);
    }

    for t in v {
        t.join().unwrap();
    }
}