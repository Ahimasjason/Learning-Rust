use std::fs; // file system
use std::error::Error;


#[derive(Debug)]
pub struct Config {
    query_str: String,
    filename :String,
}
impl Config {
    // add code here
    // new will return Result enum 
    
    pub fn new(args:&[String])->Result<Config, &'static str>{
        if args.len()< 3{
            return Err("Not enough arguments");
        }
        let query = args[1].clone(); // creating new location in menmory with that reference
        let f_name = args[2].clone();
        Ok(Config{query_str:query, filename:f_name})
    }

    pub fn cng_config(&mut self){
        // test code with mutable refeerence 
        self.query_str = String::from("Different");
    }
}


pub fn run(config:Config) -> Result<(),Box<dyn  Error>>{
    /// dyn --> Dynamic
    /// Box<dyn Error> means the function will return a type that implements the Error trait, 
    /// but we don’t have to specify what particular type the return value will be. 
    /// This gives us flexibility to return error values that may be of different types in different 
    /// error cases. The dyn keyword is short for “dynamic.”
    let content = fs::read_to_string(config.filename)?;
    println!("\n{}",content);
    Ok(())
}