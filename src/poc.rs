#![allow(unused)]

extern crate git2;
extern crate regex;

use clap::Parser;
use git2::Repository;
use git2::{Cred, Error, RemoteCallbacks};
use regex::Regex;
use std::env;
use std::path::Path;

fn url_clone() {
    let url = "https://github.com/alexcrichton/git2-rs";
    Repository::clone(url, "/Users/kadvanv/tmp").expect("failed to clone repo");
    // YAY!!! this FINALLY works

    let repo = match Repository::clone(
        url,
        Path::new(&format!("{}/temp/rustlang", env::var("HOME").unwrap())),
    ) {
        // YAY!!! this FINALLY works
        Ok(repo) => repo,
        Err(e) => panic!("failed to clone: {}", e),
    };
}

pub fn ssh_clone(host: &str, into: &Path) {
    // println!(
    //     "Path: {:?}",
    //     Path::new(&format!("{}/.ssh/id_ecdsa", env::var("HOME").unwrap()))
    // );
    // ~/.ssh/id_ecdsa
    // // Prepare callbacks.
    let mut callbacks = RemoteCallbacks::new();
    callbacks.credentials(|_url, username_from_url, _allowed_types| {
        println!("UserName: {}", username_from_url.unwrap());
        Cred::ssh_key(
            username_from_url.unwrap(),
            None,
            Path::new(&format!("{}/.ssh/id_ecdsa", env::var("HOME").unwrap())),
            None,
        )
    });

    // Prepare fetch options.
    let mut fo = git2::FetchOptions::new();
    fo.remote_callbacks(callbacks);

    // Prepare builder.
    let mut builder = git2::build::RepoBuilder::new();
    builder.fetch_options(fo);

    // Clone the project.
    let result = builder.clone(host, into);
    //
    match result {
        Ok(content) => {
            println!("Repo cloned to {:?}", into);
        }
        Err(error) => {
            println!("Oh noes: {}", error);
        }
    }
}
