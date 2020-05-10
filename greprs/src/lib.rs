use std::path;
use std::env;
use std::fs;

#[derive(Debug)]
pub struct  Config {
    pub filename: String,
    pub query_string : String
}


impl Config {
    pub fn new(filename :String , query_string :String) -> Result<Config,&'static str>{
        let con = Config{filename , query_string};
        Ok(con)
    }
}

pub fn check_file_name(file_name : &String) -> bool {
    if let false = file_name.contains("."){
        println!(" file not valid from fn");
        return false;
    }
    let allowed_extn = ["txt","pdf"];
    let test : Vec<&str> = "some_text.txt".split(".").collect::<Vec<&str>>();
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