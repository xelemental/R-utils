use std::error::Error;
use clap::{App, Arg};
use std::fs::File;
use std::io::{self, BufRead, BufReader};
#[derive(Debug)]
pub struct Params {

files : Vec<String>,
number_of_lines : bool,
number_of_non_blank_lines : bool,

}
type MyResult<T> = Result<T, Box<dyn Error>>;

pub fn get_arguments_from_user()-> MyResult<Params> {
let matches = App::new("rustcat")
.version("0.1.0")
.author("ElementalX")
.about("cat in Rust")
.arg(

      Arg::with_name("files")
      .value_name("FILES")
      .help("List of Input File/s")
      .multiple(true)
      .default_value("-"),
   )

.arg(
          Arg::with_name("number")
         .long("number")
         .short("n")
         .help("number all output lines")
         .takes_value(false)
         /*
        .conflicts_with("number_of_non_blank"), // Omitted this because I did not want the program to Err if both flags are passed.
        */
   )

.arg(
           Arg::with_name("number_of_non_blank")
           .short("b")
           .long("number-nonblank")
           .help("number nonempty output lines, overrides -n")
           .takes_value(false),
   )
   
.arg(
             Arg::with_name("parseoutput")
             .short("p")
             .long("parse-arguments")
             .help("Parse the arguments and print them"),
             
    )   


      .get_matches();

Ok(Params {
    files: matches.values_of_lossy("files").unwrap(),
    number_of_lines: matches.is_present("number"),
    number_of_non_blank_lines: matches.is_present("number_of_non_blank"),
    //let printparsed = matches.is_present("parseoutput");
    //println!("{:#?}", (matches, if printparsed{ "" } else { "Cannot Parse Output"}));
})

}


pub fn run(_pr:Params) -> MyResult<()> {
    for filename in _pr.files {
        match open(&filename) {
            Err(err) => eprintln!("{}: {}", filename, err),
            Ok(file) => {
                let mut last_num = 0; 
                for (line_num, line) in file.lines().enumerate() {
                    let line = line?; 
                    if _pr.number_of_lines { 
                        println!("{:>6}\t{}", line_num + 1, line);
                    } else if _pr.number_of_non_blank_lines { 
                        if !line.is_empty() {
                            last_num += 1;
                            println!("{:>6}\t{}", last_num, line); 
                        } else {
                            println!(); 
                        }
                    } else {
                        println!("{}", line); 
                    }
                }
            }
        }
    }
    //dbg!(_pr);
    Ok(())
}


pub fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {

match filename {


   "_" => Ok(Box::new(BufReader::new(io::stdin()))),
   _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
}



}
