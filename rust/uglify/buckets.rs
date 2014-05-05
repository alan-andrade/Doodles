extern crate manifesto;

use std::sync::mpsc_queue::{Queue, Data, Empty, Inconsistent};
use std::sync::arc::UnsafeArc;
use std::comm::channel;
use std::io::Process;
use manifesto::Manifesto;

fn main () {
    let manifesto = Manifesto::new("manifesto.json");
    let file_chunks = manifesto.split(1);

    let (tx, rx) = channel();
    let queue = Queue::new();
    let arc_queue = UnsafeArc::new(queue);

    for files in file_chunks.iter() {
        let filenames = pluck_filenames(*files);

        let arc_queue = arc_queue.clone();
        spawn(proc() {
            unsafe {
                (*arc_queue.get()).push(Process::new("uglifyjs", filenames))
            }
        });
    }

    let mut i = 0;
    while i < file_chunks.len() {
        match unsafe { (*arc_queue.get()).pop() } {
            Data(d) => {
                i = i + 1;
                tx.send(d);
            },
            Empty|Inconsistent => {}
        }
    }

    for _i in range(0, file_chunks.len()) {
        let output = rx.recv().unwrap().id();//.stdout.get_mut_ref().read_to_end();
        println!("{:?}", output);
    }

}

fn pluck_filenames (files: &[Path]) -> ~[~str] {
    files.iter().map(|f| f.as_str().unwrap().to_owned()).collect::<~[~str]>()
}
