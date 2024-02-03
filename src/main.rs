use std::{env, process};

use minigrep::Config;
use minigrep::run;

fn main() {
    
    let args : Vec<String> =  env::args().collect();
    let config = Config::build(&args)
                        .unwrap_or_else(|err| {
                            eprintln!("Error Parsing arguments : {err}");
                            process::exit(1);
                        });

    if let Err(e) = run(config){
        eprintln!("Error : {e}");
        process::exit(1);
    }    
}






