use std::io::Error;
use std::path::Path;

use clap::{App, Arg};
use ruby_parser;

fn argspec() -> App<'static> {
    App::new("ruby")
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
}

fn main() -> Result<(), Error> {
    let opts = argspec().get_matches();
    //println!("{:?}", opts);
    if let Some(path) = opts.value_of("PROGRAM_FILENAME") {
        let path = Path::new(path);
        let contents = std::fs::read_to_string(path)?;
        let input = ruby_parser::lexer::Input::new(&contents);
        let program = ruby_parser::parse(input);
        println!("program {:?}", program);
    }
    Ok(())
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_arguments() {
        // Base case
        let res = argspec().try_get_matches_from(vec!["ruby"]);
        assert!(res.is_ok());

        // Program filename
        let res = argspec().try_get_matches_from(vec!["ruby", "foo.rb"]);
        assert!(res.is_ok());
        assert_eq!(res.unwrap().value_of("PROGRAM_FILENAME"), Some("foo.rb"));
    }
}
