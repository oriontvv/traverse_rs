extern crate serde;
extern crate serde_json;

use std::fs::File;
use std::path::Path;
use std::io::BufReader;

use clap::{Arg, App, ArgMatches};
use serde_json::Value;
use regex::RegexSet;

static PATH: &str = "/home/socexp/dump/socexp_aspects.dump";

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
        self.reg_exps.matches(string).matched_any()
    }
}

fn create_finder<'a>(is_regexp: bool, conditions: Vec<&'a str>) -> Box<dyn Finder + 'a> {
    if is_regexp {
        let regexps = RegexSet::new(conditions).expect("Invalid regex");
        Box::new(RegexpFinder { reg_exps: regexps })
    } else {
        Box::new(StringFinder { conditions:conditions })
    }
}

fn parse_args() -> ArgMatches<'static> {
    App::new("Simple Finder")
        .version("0.1.0")
        .about("Finder in json")
        .arg(Arg::with_name("path")
                .long("path")
                .default_value(PATH)
                .help("path to dump file"))
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
    let args = parse_args();
    let is_regexp = args.is_present("regexp");
    let file_path = Path::new(args.value_of("path").unwrap());
    let conditions: Vec<_> = args.values_of("conditions").unwrap().collect();
    
    let finder = create_finder(is_regexp, conditions);

    let json_file = File::open(file_path).expect("file not found");
    let reader = BufReader::new(json_file);
    let mut data: Value = serde_json::from_reader(reader).expect("error while reading json");

    finder.find(String::from(""), &data["aspects"].take());
}
