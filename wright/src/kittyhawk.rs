extern crate wright;
extern crate regex;
extern crate clap;

use clap::{Arg, App, AppSettings, SubCommand};
use wright::version::VERSION;

fn main() { 
    let matches = App::new("kittyhawk")
        .version(VERSION)
        .about("Kitty hawk JVM implementation.")
        .get_matches();
    unimplemented!()
}