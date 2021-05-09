use clap::{App, Arg, ArgMatches};

pub fn get_cli_matches() -> ArgMatches<'static> {
    /* Move this out to a function that returns a config struct with all the
     * options */
    /* or just return the ArgMatches object for clarity */
    return App::new("Vikt-rs")
        //.version("0.01")
        .version(env!("CARGO_PKG_VERSION"))
        .author("Imbus64")
        .about("Keeps a log of your body weight")
        .arg(
            Arg::with_name("weight")
                .short("a")
                .long("add")
                .help("Add weight to log"),
        )
        .arg(
            Arg::with_name("list")
                .short("l")
                .long("list")
                .conflicts_with("weight")
                .help("Print all entries"),
        )
        .arg(
            Arg::with_name("raw")
                .long("raw")
                .conflicts_with_all(&["list", "plain"])
                .help("Print raw log file to stdout"),
        )
        .arg(
            Arg::with_name("plain")
                .long("plain")
                .conflicts_with_all(&["list", "raw"])
                .help("Print all entries without pretty table formatting"),
        )
        .get_matches();
}
