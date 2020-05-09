

#[derive(Debug)]
pub struct  Config {
    pub filename: String,
    pub query_string : String
}


impl Config {
    pub fn new(filename :String , query_string :String) -> Result<Config,&'static str>{
        // match filename {
        //     Some  => println!("Something got"),
        //     None  => println!("Nothing Got "),
        // }
        // if 2 < 3 {
        //     return Err("Some thing went wrong")
        // }
        let con = Config{filename , query_string};
        Ok(con)
    }
}

pub fn check_file_name(file_name : &String) -> bool {
    match file_name.contains(".") {
        false => return false,
        _ => (),
    }
    let allowed_extn = ["txt","pdf"];
    let test : Vec<&str> = "some_text.txt".split(".").collect::<Vec<&str>>();
    let file_ext = &test[test.len()-1];
    match allowed_extn.into_iter().find(|a| a == &file_ext){
        Some(_) => return true,
        None => return false,
    }
}