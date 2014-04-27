extern crate serialize;
// Manifesto
//
// This library will look for "manifesto.json" and dig into the
// "manifesto" key. The value should be an array of files.
//
// This is an example of how the manifesto.json should look.
//
// "manifesto": [
//      "js/foo.js",
//      "js/bar.js",
//      "js/*.js"
// ]
//
// You can use wildcards.
//
// The manifesto gives us information about which files should be
// printed at the beginning of the resulting file.

#[crate_id = "manifesto#0.1"]
#[crate_type = "rlib"]
#[feature(macro_rules)]

extern crate glob;

use std::io::fs::File;
use serialize::json;
use glob::glob;

static FILENAME : &'static str  = "manifesto.json";
static KEYNAME: &'static str    = "manifesto";

struct Manifesto {
    list: Vec<~Path>
}

impl Manifesto {
    fn new () -> Manifesto {
        let json_str: &str = match File::open(&Manifesto::path()).read_to_str() {
            Ok(s) => s,
            Err(e) => fail!("{}", e)
        };
        Manifesto { list: Manifesto::read(json_str) }
    }

    fn read (data: &str) -> Vec<~Path> {
        let decoded_json = json::from_str(data).unwrap();

        let json_leaf = match decoded_json.find(&KEYNAME.to_owned()) {
            Some(i) => i,
            None => fail!("{} doesn't contain the key {}", FILENAME, KEYNAME)
        };

        let filenames_list = match json_leaf.as_list() {
            Some(d) => { d },
            None => { fail!("The value of {} doesn't contain an array.", KEYNAME) }
        };

        let filenames = filenames_list.iter().map(|json_filename| {
            let filename = match json_filename.as_string() {
                Some(f) => { f },
                None => { fail!("Array contents aren't strings.") }
            };
            ~Path::new(filename)
        }).collect();

        Manifesto::expand(filenames)
    }

    fn expand(paths: ~[~Path]) -> Vec<~Path> {
        let mut collector = Vec::new();
        let mut is_global = false;

        for path in paths.iter() {
            for expanded_path in glob(path.as_str().unwrap()) {
                is_global = true;
                collector.push(~expanded_path.clone());
            }

            if is_global == false {
                collector.push((*path).clone());
            }

            is_global = false
        }

        collector
    }

    fn path () -> Path {
        Path::new(FILENAME)
    }

    fn check () -> bool {
        Manifesto::path().exists()
    }
}

#[cfg(test)]
mod test {
    use Manifesto;
    use FILENAME;
    use std::io::fs::File;

    #[test]
    fn test_check () {
        delete_json();
        assert_eq!(Manifesto::check(), false);
        create_json();
        assert_eq!(Manifesto::check(), true);
        delete_json();
    }

    #[test]
    fn test_read () {
        let json_str = &"{ \"manifesto\": [\"js/jquery.js\"] }";
        let read = Manifesto::read(json_str);
        let test_path = ~Path::new("js/jquery.js");
        assert_eq!(read.get(0).filename(), test_path.filename());


        let json_str = &"{ \"manifesto\": [\"js/jquery.js\", \"js/d3.js\"] }";
        let read = Manifesto::read(json_str);
        let test_path = ~Path::new("js/jquery.js");
        assert_eq!(read.get(0).filename(), test_path.filename());
        let test_path = ~Path::new("js/d3.js");
        assert_eq!(read.get(1).filename(), test_path.filename());


        let json_str = &"{ \"manifesto\": [] }";
        assert!(Manifesto::read(json_str) == vec!());
    }

    #[test]
    #[should_fail]
    fn test_read_fails_wrong_key () {
        let json_str = &"{ \"wrong_key\": [\"js/jquery.js\", \"js/d3.js\"] }";
        Manifesto::read(json_str);
    }

    #[test]
    #[should_fail]
    fn test_read_fails_when_no_array () {
        let json_str = &"{ \"manifesto\": \"js/jquery.js\" }";
        Manifesto::read(json_str);
    }

    #[test]
    fn test_expand () {
        let glob_path = ~Path::new("js/*.js");
        let man = Manifesto::expand(~[glob_path]);

        let test_path = ~Path::new("js/d3.js");
        assert_eq!(man.get(0).filename(), test_path.filename());

        let test_path = ~Path::new("js/jquery-2.1.0.js");
        assert_eq!(man.get(1).filename(), test_path.filename());

        let test_path = ~Path::new("js/raphael.js");
        assert_eq!(man.get(2).filename(), test_path.filename());

        let test_path = ~Path::new("js/underscore.js");
        assert_eq!(man.get(3).filename(), test_path.filename());
    }

    #[test]
    fn test_read_with_expand () {
        let json_str = &"{ \"manifesto\": [\"js/jquery.js\", \"js/*.js\"] }";
        let paths = Manifesto::read(json_str);

        let test_path = ~Path::new("js/jquery.js");
        assert_eq!(paths.get(0).filename(), test_path.filename());

        let test_path = ~Path::new("js/d3.js");
        assert_eq!(paths.get(1).filename(), test_path.filename());

        let test_path = ~Path::new("js/jquery-2.1.0.js");
        assert_eq!(paths.get(2).filename(), test_path.filename());

        let test_path = ~Path::new("js/raphael.js");
        assert_eq!(paths.get(3).filename(), test_path.filename());

        let test_path = ~Path::new("js/underscore.js");
        assert_eq!(paths.get(4).filename(), test_path.filename());
    }

    #[test]
    fn test_split_for_parallel_compilation () {
        //macro_rules! m (
            //($n:$expr) =>
            //(Manifesto {
                //list: range(0, $n).map(|i|
                        //Path::new("js/" + i.to_str()))
                          //.collect() })
            //);

        //let m = m!(1);
        //assert_eq!(m.len(), 1);
    }

    #[test]
    fn test_new () {
        create_json();
        let manifesto = Manifesto::new();
        let test_path = Path::new("js/jquery.js");
        assert_eq!(manifesto.list.get(0).filename(), test_path.filename());
        delete_json();
    }

    fn create_json () {
        match File::create(&Path::new(FILENAME)) {
            Ok(mut f) => {
                let content = bytes!("{\"manifesto\": [\"js/jquery.js\"]}");
                f.write(content).unwrap()
            },
            Err(e) => { fail!("{}", e) }
        };
    }

    fn delete_json () {
        use std::io::process::Process;
        match Process::new("rm", [FILENAME.to_owned()]) {
            Ok(mut child) => { child.wait(); },
            Err(_) => { println!("manifest.json didn't exist.") }
        };
    }
}
