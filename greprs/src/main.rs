extern crate greprs;
use std::io::prelude::*;
use std::env::args;
use std::io::prelude::*;
use std::process;

use greprs::Config;

fn main(){
    // Accepting args from command line
    let args: Vec<String> = args().collect();
    let mut stderr = std::io::stderr();
    let con = Config::new(String::from("File name"),String::from("find")).unwrap_or_else( |err| 
        {
            writeln!(
                &mut stderr,
                "problem parsing args : {} ",
                err
            ).expect("Could not write to stderr");
            process::exit(1);
    });
   
    match greprs::check_file_name(&con.filename) {
        true => println!(" File Exist"),
        false => {
            writeln!(
                &mut stderr,
                "Not a valid file format expected (txt,pdf)"
            ).expect("Could not write to stderr");
            process::exit(1);
        }
    }
    
    println!("{:?}",con);
    println!("{:?}",args);
}

