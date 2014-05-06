extern crate manifesto;

use std::comm::channel;
use std::io::Process;
use std::io::fs::File;
use manifesto::Manifesto;

fn main () {
    let manifesto = Manifesto::new("manifesto.json");
    let file_chunks = manifesto.split(3);

    let (tx, rx) = channel();

    for (num, files) in file_chunks.iter().enumerate() {
        println!("chunk: {}", num+1);
        let filenames = pluck_filenames(*files);
        println!("filenames: {:?}", filenames);
        tx.send(Process::new("uglifyjs", filenames));
    }

    let mut file = File::create(&Path::new("with_channels.js"));
    for _i in range(0, file_chunks.len()) {
        let mut pro = rx.recv().unwrap();
        let msg = pro.wait_with_output();
        file.write(msg.output.as_slice());
    }
}

fn pluck_filenames (files: &[Path]) -> ~[~str] {
    files.iter().map(|f| f.as_str().unwrap().to_owned()).collect::<~[~str]>()
}
