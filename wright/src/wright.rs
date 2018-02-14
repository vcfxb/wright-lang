extern crate wright;
extern crate clap;
use clap::App;
use clap::Arg;
use wright::run;
use wright::version::VERSION;

fn main() {
    let app = App::new("Wright")
        .version(VERSION)
        .author("Antonia Calia-Bogan (github.com/Alfriadox)")
        .about("The Wright programming language interpreter.")
        .arg(Arg::with_name("help")
            .short("h")
            .long("help")
            .help("Prints help information."))
        .arg(Arg::with_name("version")
            .short("v")
            .takes_value(false)
            .long("version")
            .help("Prints wright's version."))
        .get_matches();
}
