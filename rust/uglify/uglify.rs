use std::io::fs::{walk_dir, File};
use std::io::{Process,TempDir};

//static CORES: uint = 4;

fn main () {
    let js_dir = Path::new("js");
    let mut paths = walk_dir(&js_dir).unwrap();
    let mut uglies = Vec::new();

    for path in paths {
        uglies.push(UProc::new([path.as_str().unwrap().to_owned()]));
    }

    let mut file = File::create(&Path::new("mylib.js"));

    for ugly in uglies.mut_iter() {
        ugly.ps.wait();
        file.write(ugly.read()).unwrap()
    }
}

struct TempFile {
    file: File
}

impl TempFile {
    fn new() -> TempFile {
        let temp_path = TempDir::new(&"split").unwrap();
        let temp_file_name = temp_path.path().as_str().unwrap() + "test";
        let temp_file_path = Path::new(temp_file_name);

        let temp_file = match File::create(&temp_file_path) {
            Ok(f) => f,
            Err(e) => fail!("{}", e)
        };

        TempFile { file: temp_file }
    }

    fn fullname(&self) -> ~str {
        self.file.path().as_str().unwrap().to_owned()
    }
}

struct UProc {
    file: TempFile,
    ps: Process
}

impl UProc {
    fn new (load: &[~str]) -> ~UProc {
        let file = TempFile::new();
        let mut options = vec!(~"-me", ~"-o", file.fullname());
        options.push_all(load);

        match Process::new("uglifyjs", options.as_slice()) {
            Ok(ps) => { ~UProc { file: file, ps: ps } },
            Err(e) => { fail!("\nPlease install uglifyjs with 'npm -g install ugligy-js'.\n{}", e) }
        }
    }

    fn read(&self) -> ~[u8] {
        let mut reader = File::open(self.file.file.path()).unwrap();
        reader.read_to_end().unwrap()
    }
}
