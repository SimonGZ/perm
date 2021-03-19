use regex::Regex;
use std::fs;
use std::io::{self, Read, Write};
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(
    name = "Curling Iron",
    about = "A simple tool for converting straight/dumb quotes and apostrophes into their smart/curly equivalents."
)]
struct Opt {
    /// Input file, pass a dash ("-") to receive stdin
    #[structopt(parse(from_os_str))]
    input: PathBuf,

    /// Output file, stdout if not present
    #[structopt(parse(from_os_str))]
    output: Option<PathBuf>,
}

fn replace_apostrophes(text: String, smarten: bool) -> String {
    if smarten {
        text.replace("'", "’")
    } else {
        text.replace("’", "'")
    }
}

fn replace_quotes(text: String, smarten: bool) -> String {
    let re_quotes: Regex = Regex::new(r#"([\w,.?!)]|^)(")"#).unwrap();
    if smarten {
        let half_quoted = re_quotes.replace_all(&text, "$1”");
        half_quoted.replace("\"", "“")
    } else {
        text.replace("“", "\"").replace("", "\"")
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

    content = replace_apostrophes(content, true);
    content = replace_quotes(content, true);
    let output_text: String = content;

    match opt.output {
        Some(outfile) => fs::write(outfile, output_text).expect("Unable to write file."),
        None => {
            let stdout = io::stdout();
            let mut handle = io::BufWriter::new(stdout);
            writeln!(handle, "{}", output_text).expect("Unable to write to buffer.");
        }
    }
}
