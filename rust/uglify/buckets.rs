#![crate_id = "buckets#0.0.1"]

extern crate manifesto;
//use std::sync::mpsc_queue::{Queue, Data};
//use std::sync::arc::UnsafeArc;
//use std::comm::channel;
use manifesto::Manifesto;

fn main () {
    if !Manifesto::check() {
        fail!("Please create a manifesto.json");
    }

    let manifesto = Manifesto::new();
    let cores = 4;

    for files in manifesto.split(cores).iter() {
        println!("{:?}", files);
        println!("====");
    }

    //let (tx, ry) = channel();
    //let mut q = Queue::new();

    //let q = UnsafeArc::new(q);

    //tx.send(1);
    //ry.recv();

    //let q1 = q.clone();
    //spawn(proc() {
        //unsafe { (*q1.get()).push(1); }
    //});

    //let q1 = q.clone();
    //spawn(proc() {
        //unsafe { (*q1.get()).push(2); }
    //});

    //for i in range(0,5) {
        //let z = unsafe { (*q.get()).pop() };
        //match z {
            //Data(d) => { println!("{}", d) },
            //_ => { println!("no match {}", i+1) }
        //}
    //}
}
