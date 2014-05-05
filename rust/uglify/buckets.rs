extern crate manifesto;

use std::sync::mpsc_queue::{Queue, Data, Empty, Inconsistent};
use std::comm::channel;
use std::io::Process;
use std::io::fs::File;
use manifesto::Manifesto;

fn main () {
    let manifesto = Manifesto::new("manifesto.json");
    let file_chunks = manifesto.split(1);

    let (tx, rx) = channel();
    let mut queue = Queue::new();

    for files in file_chunks.iter() {
        let filenames = pluck_filenames(*files);
        queue.push(Process::new("uglifyjs", filenames));
    }

    let mut i = 0;
    while i < file_chunks.len() {
        match queue.pop() {
            Data(d) => {
                i = i + 1;
                match d {
                    Ok(mut pr) => {
                        tx.send(pr);
                    },
                    Err(_) => {}
                }
            },
            Empty|Inconsistent => {}
        }
    }

    let mut file = File::create(&Path::new("with_channels.js"));
    for _i in range(0, file_chunks.len()) {
        let mut pro = rx.recv();
        let msg = pro.wait_with_output();
        file.write(msg.output.as_slice());
    }

    let mut final = File::create(&Path::new("with_channels_mangled.js"));
    match Process::new("uglifyjs", &[
                       "with_channels.js".to_owned(),
                        "-m".to_owned()]) {
        Ok(mut p) => {
            final.write(p.wait_with_output().output.as_slice());
        }
        Err(f) => { fail!("{}", f) }
    }
}

fn pluck_filenames (files: &[Path]) -> ~[~str] {
    files.iter().map(|f| f.as_str().unwrap().to_owned()).collect::<~[~str]>()
}
