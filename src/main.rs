#![feature(path_ext)]
#![feature(collections)]

#[macro_use]
extern crate log;
extern crate git2;
extern crate iron;
extern crate toml;

use std::env;
use std::io::Read;
use std::fs::{self, PathExt, File};
use std::path::PathBuf;
use std::error::Error;

use git2::Repository;

use iron::prelude::*;

/// Contains various logger implementations for Grist
mod loggers;
/// Contains Iron server functions
mod servers;
/// Contains code related to configuration file parsing
mod config;

fn main() {

    // try to parse the config file at ~/.gristconfig.toml; otherwise, use defaults
    let configs = File::open("~/.gristconfig.toml")
        .and_then(|mut f: File| {
            let mut s = String::new();
            try!(f.read_to_string(&mut s));
            Ok(s) })
        .map_err(|err| String::from_str(err.description()) )
        .and_then(|s: String| config::parse_toml(s.as_ref()))
        .unwrap_or(config::DEFAULTS.clone());

    // set log level from configs
    let _ = if configs.debug {
        log::set_logger(|max_log_level| {
            max_log_level.set(log::LogLevelFilter::Debug);
            Box::new(loggers::DebugLogger)
        });
    } else {
        log::set_logger(|max_log_level| {
            max_log_level.set(log::LogLevelFilter::Info);
            Box::new(loggers::DefaultLogger)
        });
    };

    let roots = configs.roots.unwrap_or(
        // get the current working dir, assume that we are in a valid directory
        vec![env::current_dir().unwrap()]
    );

    info!("Starting Grist in {:?}", roots);

    let repos: Vec<Repository> = roots.iter().flat_map(|ref root: &PathBuf| {
        fs::read_dir(root) // walk the root dir
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
    }).collect();

    info!("Discovered {} repositories.", repos.len());

    // serve sample hello world page for now
    Iron::new(servers::hello_world).http("localhost:3000").unwrap();

}
