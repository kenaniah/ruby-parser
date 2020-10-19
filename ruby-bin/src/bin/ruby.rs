use clap::{App, Arg};

fn main() {
    let opts = App::new("ruby")
        .version("0.0.0-dev1")
        .arg(
            Arg::new("dump")
                .long("dump")
                .value_name("dump_list")
                .takes_value(true)
                .possible_values(&["insns", "parsetree", "parsetree_with_comment"])
                .multiple(true)
                .use_delimiter(true)
                .about("dumps debug information"),
        )
        .arg(
            Arg::new("execute")
                .short('e')
                .takes_value(true)
                .value_name("code")
                .multiple_occurrences(true)
                .conflicts_with("PROGRAM_FILENAME")
                .about("a line of ruby code to be executed, can be specified multiple times"),
        )
        .arg(Arg::new("PROGRAM_FILENAME").about("path to a ruby file to be executed"))
        .get_matches();
    println!("{:?}", opts);
}
