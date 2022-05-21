mod git;
extern crate regex;

use git::make_url;
use git::valid_ssh_url;
use git::write_location;

fn main() {
    let url = std::env::args().nth(1).expect("no url given");
    let should_open = std::env::args().nth(2);
    if valid_ssh_url(&*url) {
        match should_open {
            Some(value) => {
                if value.eq("o") {
                    try_open(url)
                }
            }
            _ => {
                let into = write_location(&*url);
                let output = into.into_os_string().into_string().expect("failed");
                println!("{}", output)
            }
        }
    } else {
        println!("{}", &*url)
    }
}

fn try_open(value: String) {
    let url = make_url(&*value);
    println!("{}", url);
}
