mod git;
mod poc;
extern crate git2;
extern crate regex;

use git::valid_ssh_url;
use git::write_location;
use poc::ssh_clone;

fn main() {
    // println!("Welcome to dolly 0.1.0.");
    let url = std::env::args().nth(1).expect("no url given");
    if valid_ssh_url(&*url) {
        let into = write_location(&*url);
        ssh_clone(&*url, &*into);
        let output = into.into_os_string().into_string().expect("failed");
        println!("{}", output)
    }
}
