use std::io;
use clap::{self, App, Arg, ArgMatches};
use rustotf::cli;

fn main() -> io::Result<()> {
    let matches = app().get_matches();
    if let Some(input_path) = matches.value_of("input") {
        let ttc_indices = parse_arg_ttc_indices(&matches);
        if matches.is_present("list") {
            cli::list_tables(input_path, ttc_indices)?;
        } else {
            let tables = parse_arg_tables(&matches);
            cli::print_tables(input_path, ttc_indices, tables);
        }
    }
    Ok(())
}

fn app() -> App<'static> {
    let arg_help = Arg::new("help")
        .long("help")
        .short('h')
        .takes_value(false)
        .about("Print this help message and exit.");
    let arg_version = Arg::new("version")
        .long("version")
        .short('v')
        .takes_value(false)
        .about("Print version information and exit.");
    let arg_list = Arg::new("list")
        .long("list")
        .short('l')
        .takes_value(false)
        .about("Print some basic information about each table.");
    let arg_tables = Arg::new("tables")
        .long("tables")
        .short('t')
        .takes_value(true)
        .value_name("TABLE")
        .about("Specify a table to dump. If not specified, then all tables will be dumpled.");
    let arg_output = Arg::new("output")
        .long("output")
        .short('o')
        .takes_value(true)
        .value_name("FILE")
        .about("Set output file path.");
    let arg_ttc_indices = Arg::new("ttc_indices")
        .long("ttc-indices")
        .short('y')
        .takes_value(true)
        .value_name("N1[,N2,...]")
        .about("Select font number(s) for OpenType Collection, starting from 0. If not specified, then all subfonts will be dumpled.");
    let arg_input = Arg::new("input")
        .value_name("INPUT")
        .about("Specify the input font file.")
        .required_unless_present_all(&["help", "version"]);
    App::new(clap::crate_name!())
        .author(clap::crate_authors!())
        .version(clap::crate_version!())
        .arg(arg_help)
        .arg(arg_version)
        .arg(arg_list)
        .arg(arg_tables)
        .arg(arg_output)
        .arg(arg_ttc_indices)
        .arg(arg_input)
}

fn parse_arg_ttc_indices(matches: &ArgMatches) -> Vec<usize> {
    match matches.value_of("ttc_indices") {
        Some(value) => value
            .split(',')
            .map(|s| match s.parse() {
                Ok(n) => n,
                Err(_) => panic!("Invalid ttc index {:?}.", s),
            })
            .collect(),
        None => Vec::new(),
    }
}

fn parse_arg_tables(matches: &ArgMatches) -> Vec<&str> {
    match matches.value_of("tables") {
        Some(value) => value.split(',').collect(),
        None => Vec::new(),
    }
}
