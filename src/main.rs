mod git;
mod poc;
extern crate git2;
extern crate regex;

use git::make_url;
use git::valid_ssh_url;
use git::write_location;
use poc::ssh_clone;

fn main() {
    // println!("Welcome to dolly 0.1.0.");
    let url = std::env::args().nth(1).expect("no url given");
    let shouldOpen = std::env::args().nth(2);
    if valid_ssh_url(&*url) {
        match shouldOpen {
            Some(value) => {
                if (value.eq("o")) {
                    try_open(url)
                }
            }
            _ => {
                let into = write_location(&*url);
                ssh_clone(&*url, &*into);
                let output = into.into_os_string().into_string().expect("failed");
                println!("{}", output)
            }
        }
    }
}

fn try_open(value: String) {
    let url = make_url(&*value);
    println!("{}", url);
}
