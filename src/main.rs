extern crate serde;
extern crate serde_json;

use std::time::Instant;
use std::fs::File;
use std::path::Path;

use clap::{Arg, App, ArgMatches};
use serde_json::Value;
use regex::RegexSet;


static PATH: &str = "./test.json";


trait Finder {
    fn check(&self, string: &str) -> bool;

    fn find(&self, path: String, data: &Value) {
        if data.is_object() {
            let obj = data.as_object().unwrap();
            for (key, value) in obj.into_iter() {
                let new_path = path.clone() + "." + key;
                self.find(new_path, value);
            }
        } else if data.is_array() {
            let array = data.as_array().unwrap();
            for (index, value) in array.into_iter().enumerate() {
                let new_path = format!("{}[{}]", path.clone(), index);
                self.find(new_path, value);
            }
        } else if data.is_string() {
            let string = data.as_str().unwrap();
            if self.check(string) {
                println!("{} {}", string, path);
            }
        }
    }
}

struct StringFinder<'a> {
    pub conditions: Vec<&'a str>
}

impl Finder for StringFinder<'_> {
    fn check(&self, string: &str) -> bool {
        self.conditions.contains(&string)
    }
}

struct RegexpFinder {
    pub reg_exps: RegexSet
}

impl Finder for RegexpFinder {
    fn check(&self, string: &str) -> bool {
        false
    }
}

// fn create_finder(is_regexp: bool, conditions: Vec<&str>) -> Box<dyn Finder> {
//     if is_regexp {
//         Box::new(RegexpFinder { reg_exps: RegexSet::new(conditions).expect("Invalid regex") })
//     } else {
//         Box::new(StringFinder { conditions: conditions })
//     }
// }

fn parse_args() -> ArgMatches<'static> {
    App::new("Simple Finder")
        .version("0.1.0")
        .about("Finder in json")
        .arg(Arg::with_name("regexp")
                .long("regexp")
                .takes_value(false)
                .help("use regexp"))
        .arg(Arg::with_name("conditions")
                .long("conditions")
                .takes_value(true)
                .required(true)
                .multiple(true)
                .help("list of conditions"))
        .get_matches()
}

fn main() {
    let start = Instant::now();

    let args = parse_args();
    let is_regexp = args.is_present("regexp");
    let conditions: Vec<_> = args.values_of("conditions").unwrap().collect();
    
    // let finder = create_finder(is_regexp, conditions);

    let file_path = Path::new(PATH);
    let json_file = File::open(file_path).expect("file not found");
    let data: Value = serde_json::from_reader(json_file).expect("error while reading json");

    // finder.find(String::from(""), &data["aspects"].take());

    let elapsed = start.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}
