use clap::{App, Arg};

fn main() {
    let matches = App::new("rustecho")
    .version("0.1.0")
    .author("ElementalX")
    .about("echo")
    .bin_name("rustecho")
    .usage("rustecho <SOMETEXTHERE>")
        .arg(
            Arg::with_name("outputtext")
                .value_name("TEXT")
                .help("Input text")
                .required(true)
                .min_values(1),
        )
        .arg(
            Arg::with_name("omitnewline")
                .short("n")
                .help("Do not print newline")
                .takes_value(false),
        )
        .arg(
             Arg::with_name("parseoutput")
             .short("p")
             .help("Parse the arguments and print them"),
             
      )
        .get_matches();

    let text = matches.values_of_lossy("outputtext").unwrap();
    let omit_newline = matches.is_present("omitnewline");
   // let parsedargs = matches.values_of_lossy("parseoutput").unwrap();
    let printparsed = matches.is_present("parseoutput");
    println!("{:#?}", (matches, if printparsed{ "" } else { "Cannot Parse Output"}));
    print!("{}{}", text.join(" "), if omit_newline{ "" } else { "\n"});

}
