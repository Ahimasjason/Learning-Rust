use std::path;
use std::env;
use std::fs;

#[derive(Debug)]
pub struct  Config {
    pub filename: String,
    pub query_string : String
}


impl Config {
    pub fn new(mut args: std::env::Args) -> Result<Config,&'static str>{
        // let con = Config{filename , query_string};
        args.next(); //skiping the curr dir

        let query_string = match args.next() {
            Some(i) => i,
            None => return Err(" Didn't get query string")
        };

        let filename = match args.next() {
            Some(f_name) => f_name ,
            None => return Err("didn't get file name from query string")
        };
        let con = Config{filename , query_string};
        Ok(con)
    }
}

pub fn check_file_name(file_name : &String) -> bool {
    
    if let false = file_name.contains("."){
        return false;
    }
    let allowed_extn = ["txt","pdf"];
    let test : Vec<&str> = file_name.split(".").collect::<Vec<&str>>();
    let file_ext = &test[test.len()-1];
    match allowed_extn.into_iter().find(|a| a == &file_ext){
        Some(_) => return true,
        None => return false,
    }
}


pub fn file_exists(file_name : &String) -> bool {
    match file_name.contains("/"){
        true => {
            println!("f{}",file_name);
            path::Path::new(&file_name).exists()
        },
        false => {
            let mut curr_dir = env::current_dir().expect("unable to locate current dir");
            curr_dir.push(&file_name);
            path::Path::new(&curr_dir).exists()
        }
    }
    
}