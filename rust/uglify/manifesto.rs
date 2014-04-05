extern crate serialize;
use std::io::fs::File;
use serialize::json;

static FILENAME : &'static str  = "manifesto.json";
static KEYNAME: &'static str    = "manifesto";

struct Manifesto {
    list: ~[~str]
}

impl Manifesto {
    fn path () -> Path {
        Path::new(FILENAME)
    }

    fn check () -> bool {
        Manifesto::path().exists()
    }

    fn read (data: &str) -> ~[~str] {
        let json = json::from_str(data).unwrap();

        let item = match json.find(&KEYNAME.to_owned()) {
            Some(j) => j,
            None => fail!("{} doesn't contain the key {}", FILENAME, KEYNAME)
        };

        let list = item.as_list().unwrap_or_else(proc(){
            fail!("The value of {} doesn't contain an array.", KEYNAME)
        });

        list.iter().map(proc(filename){
            filename.as_string().unwrap_or_else(proc(){
                fail!("Array contents aren't strings.");
            }).to_owned()
        }).collect()
    }

    fn new () -> Manifesto {
        let json_str: &str = match File::open(&Manifesto::path()).read_to_str() {
            Ok(s) => s,
            Err(e) => fail!("{}", e)
        };

        Manifesto { list: Manifesto::read(json_str) }
    }
}

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
    assert_eq!(Manifesto::read(json_str), ~[~"js/jquery.js"]);

    let json_str = &"{ \"manifesto\": [\"js/jquery.js\", \"js/d3.js\"] }";
    assert_eq!(Manifesto::read(json_str), ~[~"js/jquery.js", ~"js/d3.js"]);

    let json_str = &"{ \"manifesto\": [] }";
    assert_eq!(Manifesto::read(json_str), ~[]);
}

#[test]
fn test_new () {
    create_json();
    let manifesto = Manifesto::new();
    assert_eq!(manifesto.list, ~[~"js/jquery.js"]);
    delete_json();
}

//
// Test helper functions
//
fn create_json () {
    match File::create(&Path::new(FILENAME)) {
        Ok(mut f) => {
            f.write(bytes!("{ \"manifesto\": [\"js/jquery.js\"] }")).unwrap()
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
