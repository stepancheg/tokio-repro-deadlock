use futures::future;
use futures::future::FutureExt;

extern crate httpbis_test;








use std::thread;





use futures::channel::oneshot;





use std::sync::mpsc;


use tokio::runtime::Runtime;

use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel::<u32>();
    let (shutdown_tx, shutdown_rx) = oneshot::channel();

    let t = thread::Builder::new()
        .name("servers".to_owned())
        .spawn(move || {
            tx.send(10).expect("send");

            let mut core = Runtime::new().expect("Runtime::new");

            let handle = core.handle();

            let (_shutdown_signal, shutdown_future) = futures::channel::oneshot::channel::<()>();

            handle.clone().spawn({
                let (done_tx, done_rx) = oneshot::channel::<()>();

                let done = shutdown_future.then(|_| {
                    drop(done_tx.send(()));
                    future::ready(())
                });

                handle.spawn(done);

                done_rx
            });

            println!("waiting for shutdown message");

            core.block_on(shutdown_rx).expect("run");

            println!("thread done");
        })
        .unwrap();

    thread::sleep(Duration::from_millis(10));

    let _ports = rx.recv().expect("recv");

    println!("sending shutdown");

    shutdown_tx.send(()).expect("send");

    println!("joining the thread");

    t.join().expect("thread join");

    println!("last line of test");
}
