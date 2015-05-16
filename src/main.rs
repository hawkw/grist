#![feature(path_ext)]

#[macro_use]
extern crate log;
extern crate git2;
extern crate iron;
extern crate toml;

use std::env;
use std::fs::{self, PathExt};

use git2::Repository;

use iron::prelude::*;

/// Contains various logger implementations for Grist
mod loggers;
/// Contains Iron server functions
mod servers;
/// Contains code related to configuration file parsing
mod config;

fn main() {

    // set the default logger
    // TODO: check cfg flag for verbosity level
    let _ = log::set_logger(|max_log_level| {
            max_log_level.set(log::LogLevelFilter::Info);
            Box::new(loggers::DefaultLogger)
        });

    // get the current working dir, assume that we are in a valid directory
    let root = env::current_dir().unwrap(); // TODO: allow root to be set from cfg file
    info!("Starting Grist in {}", root.display());

    let repos: Vec<Repository> = fs::read_dir(root) // walk the root dir
        .unwrap()
        .filter_map( |entry| match entry {
            Ok(ref dir) if dir.path().is_dir() => { // found a dir
                debug!("Found directory: {:?}", dir.path());
                match Repository::init(dir.path()) { // attempt to open dir as git repo
                    Ok(repo) => { info!("Found repository: {:?}", dir.path()); Some(repo) },
                    Err(why) => { warn!("Failed to open {:?}: {}", dir.path(), why); None }
                }
            },
            Err(why) => { warn!("Could not read entry: {}", why); None },
            Ok(ref e)=> { debug!("{:?} is not a directory.", e.path()); None }
        })
        .collect();

    info!("Discovered {} repositories.", repos.len());

    // serve sample hello world page for now
    Iron::new(servers::hello_world).http("localhost:3000").unwrap();

}
