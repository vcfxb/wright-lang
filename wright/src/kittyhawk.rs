extern crate wright;
extern crate regex;
extern crate clap;

use clap::{Arg, App, AppSettings, SubCommand};
use wright::version::VERSION;

fn main() { 
    let matches = App::new("kittyhawk")
        .setting(AppSettings::ColorAlways)
        .version(VERSION)
        .author("Antonia Calia-Bogan (github.com/Alfriadox)")
        .about("A JVM implementation in rust. Part of the Wright Programming Language.")
        .get_matches();
    unimplemented!()
}