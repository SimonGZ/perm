use regex::Regex;
use std::fs;
use std::io::{self, Read};
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(
    name = "Perm",
    about = "A simple tool for curling straight/dumb quotation marks (\"\") and apostrophes (') into their curly/smart (“”’) equivalents."
)]
struct Opt {
    /// Input file, pass a dash ("-") to receive stdin
    #[structopt(parse(from_os_str))]
    input: PathBuf,

    /// Output file, stdout if not present
    #[structopt(parse(from_os_str))]
    output: Option<PathBuf>,

    /// Straighten quotes instead of curling them
    #[structopt(short, long)]
    straighten: bool,
}

fn replace_apostrophes(text: String, straighten: bool) -> String {
    if !straighten {
        text.replace("'", "’")
    } else {
        text.replace("’", "'")
    }
}

fn replace_quotes(text: String, straighten: bool) -> String {
    let re_quotes: Regex = Regex::new(r#"([\w,.?!)]|^)(")"#).unwrap();
    if !straighten {
        let half_quoted = re_quotes.replace_all(&text, "$1”");
        half_quoted.replace("\"", "“")
    } else {
        text.replace("“", "\"").replace("”", "\"")
    }
}

fn main() {
    let opt = Opt::from_args();
    let mut content = String::new();
    if opt.input.is_file() {
        content.push_str(&std::fs::read_to_string(opt.input).expect("Could not read file."));
    } else {
        if opt.input.to_str() == Some("-") {
            let mut buffer = String::new();
            let stdin = io::stdin().read_to_string(&mut buffer);
            match stdin {
                Err(_) => panic!("Invalid text piped to function."),
                Ok(_) => content.push_str(&buffer),
            }
        } else {
            eprintln!("Error: Did not receive a valid file.");
            std::process::exit(1);
        }
    }

    content = replace_apostrophes(content, opt.straighten);
    content = replace_quotes(content, opt.straighten);
    let output_text: String = content;

    match opt.output {
        Some(outfile) => fs::write(outfile, output_text).expect("Unable to write file."),
        None => {
            print!("{}", output_text);
        }
    }
}
