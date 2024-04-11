use std::env;
use std::error::Error;
use std::fs;
use std::path::Path;
// use std::any::type_name;
use std::process;

mod config;
mod search;

fn main() {
    println!("Hello, world!");
    // let args: Vec<String> = env::args().collect();
    // dbg!(&args);
    // let query = &args[1];
    // let file_path = &args[2];
    // println!("searching for {}, file_path={}", query, file_path);

    // let contents = fs::read_to_string(Path::new(&file_path)).expect("could not found file");
    // println!("{:?}", &contents);
    // let c = search::search(&query, &contents);
    // println!("output = {:?}", c);
    let c = config::Config::build(env::args()).unwrap_or_else(|err|{
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });
    if let Err(e) = run(&c) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}

fn run(c: &config::Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(Path::new(&(c.file_path)));
    let body = match contents {
        Ok(b) => b,
        Err(e) => return Err(Box::new(e)),
    };
    let results:Vec<&str> = search::search(&c.query, &body, c.is_lower);
    // println!("{:?}",type_name(&results));
    for l in results {
        println!("found:{}",l)
    }

    Ok(())
}
