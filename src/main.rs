extern crate serde;
extern crate serde_json;

use std::time::Instant;
use std::fs::File;
use std::path::Path;

use clap::{Arg, App};
use serde_json::Value;
use regex::RegexSet;


trait Checker {
    fn check(&self, string: &str) -> bool;
}

pub struct StringChecker {
    pub conditions: Vec<str>
}

impl Checker for StringChecker {
    fn check(&self, string: &str) -> bool {
        self.conditions.contains(string)
    }
    // fn new(is_regexp: bool, conditions: &Vec<str>) -> Checker {
    // }
}

pub struct RegexpChecker  {
    pub reg_exps: RegexSet
}

fn traverse(path: String, data: &Value, checker: dyn Checker) {
    if data.is_object() {
        let obj = data.as_object().unwrap();
        for (key, value) in obj.into_iter() {
            let newpath = path.clone() + "." + key;
            traverse(newpath, value, checker: checker);
        }
    } else if data.is_array() {
        let array = data.as_array().unwrap();
        for (index, value) in array.into_iter().enumerate() {
            let newpath = path.clone() + "[" + &String::from(index.to_string()) + "]";
            traverse(newpath, value, checker);
        }
    } else if data.is_string() {
        let string = data.as_str().unwrap();
        if checker.check(string) {
            println!("{} {}", string, path);
        }
    }
}

fn main() {
    let start = Instant::now();

    let matches = App::new("Simple search")
        .version("0.1.0")
        .about("Search in json")
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
        .get_matches();

    let conditions: Vec<_> = matches.values_of("conditions").unwrap().collect();
    let is_regexp = matches.is_present("regexp");
    
    let conditions = RegexSet::new(conditions).unwrap();

    let checker = if is_regexp {
        RegexpChecker { reg_exps: RegexSet::new(conditions) }
    } else {
        StringChecker { conditions: conditions } 
    };


    let file_path = Path::new("./test.json");
    let json_file = File::open(file_path).expect("file not found");

    let mut data: Value = serde_json::from_reader(json_file).expect("error while reading json");

    traverse(String::from(""), &data["aspects"].take(), checker);

    let elapsed = start.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}
