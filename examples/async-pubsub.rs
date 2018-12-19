extern crate bus_queue;
extern crate futures;
extern crate tokio;

use bus_queue::async::*;
use std::thread::*;
use std::time;
use tokio::prelude::*;

fn main() {
    let mut async_bus: AsyncBus<u32> = AsyncBus::new(10);

    fn produce(async_bus: &mut AsyncBus<u32>) -> () {
        println!("enter producer!");
        sleep(time::Duration::from_millis(2000));
        for i in 0..40 {
            async_bus.push(i);
            sleep(time::Duration::from_millis(500));
        }
    }

    fn consume(consumers: Vec<AsyncBusReader<u32>>) -> () {
        println!("enter consumers {}!", consumers.len());
        for stream in consumers {
            let future = stream.for_each(|curr| {
                println!("fut {:?} : {}", current().id(), curr);
                futures::future::ok(())
            });
            tokio::run(future);
        }
        ()
    }

    let streams = (0..10).map(|_| async_bus.add_sub()).collect();

    let a = spawn(move || consume(streams));
    let b = spawn(move || produce(&mut async_bus));

    a.join().unwrap();
    b.join().unwrap();
}
