use std::error::Error;
use std::{env, fs};

#[derive(Debug)]
pub struct Config{
    query : String,
    file_path : String,
    ignore_case : bool 
}

impl Config {
    pub fn build(args : &[String]) -> Result<Config,&'static str>{

        if args.len() < 3{
            return Err("Not Enough arguments");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();
    
        let ignore_case = env::var("IGNORE_CASE").is_ok();
        Ok(Config{
            query,
            file_path,
            ignore_case
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs:: read_to_string(config.file_path)?;        
    
    let results = if config.ignore_case{
        search_case_insensitive(&config.query, &contents)
    }
    else{
        search(&config.query, &contents)
    };

    
    for line in results{
        println!("{line}");
    }
    
    Ok(())
}

fn search_case_insensitive<'a>(query : &str, contents : &'a str ) -> Vec<&'a str>{
    let mut text_lines = Vec::new();    
    let search_query =  query.to_lowercase();
    
    for line in contents.lines(){                       
        if line.to_lowercase().contains(&search_query) {
            text_lines.push(line);
        }
    }
    text_lines
}

fn search<'a>(query : &str, contents : &'a str ) -> Vec<&'a str>{
    let mut text_lines = Vec::new();        
    for line in contents.lines(){
        
                
        if line.contains(&query) {
            text_lines.push(line);
        }
    }
    text_lines
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_search_case_sensitve(){
        let query = "Random";
        let contents = "\
        adadadada adsadad asdsasd\n\
        asdadas asd asds asd asd ad\n\
        adsasdasdasd adasd\n\
        Some, Random, Text\n\
        adadasd asdasda\n\
        random
        ";
        assert_eq!(vec!["Some, Random, Text"], search(query, contents));
    }
    #[test]
    fn test_search_case_insensitive(){
        let query = "Random";
        let contents = "\
        adadadada adsadad asdsasd\n\
        asdadas asd asds asd asd ad\n\
        adsasdasdasd adasd\n\
        Some, Random, Text\n\
        adadasd asdasda\n\
        random
        ";
        assert_eq!(vec!["Some, Random, Text", "random"], search_case_insensitive(query, contents));
    }
}