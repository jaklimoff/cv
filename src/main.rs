use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::{BufReader, Write};

use lazy_static::lazy_static;
use tera::{Context, Tera};

use crate::models::CV;

mod models;

fn main() {
    lazy_static! {
        pub static ref TEMPLATES: Tera = {
            let mut tera = match Tera::new("templates/**/*") {
                Ok(t) => t,
                Err(e) => {
                    println!("Parsing error(s): {}", e);
                    ::std::process::exit(1);
                }
            };
            tera.autoescape_on(vec![".html", ".sql"]);
            // tera.register_filter("do_nothing", do_nothing_filter);
            tera
        };
    }

    let mut tera = match Tera::new("templates/**/*") {
        Ok(t) => t,
        Err(e) => {
            println!("Parsing error(s): {}", e);
            std::process::exit(1);
        }
    };
    tera.autoescape_on(vec![".html", ".sql"]);



    let my_cv = read_json().unwrap();
    dbg!(&my_cv.basics.first_name);

    let a = tera.render("index.html", &Context::from_serialize(&my_cv).unwrap()).unwrap();

    // let mut file = OpenOptions::new()
    //     .create(true)
    //     .open("output/index.html").unwrap();

    match write_to_file(a.as_str(), "docs/index.html") {
        Ok(()) => println!("Successfully wrote to the file."),
        Err(e) => eprintln!("Error writing to the file: {}", e),
    }

    // file.write_all(a.as_bytes()).unwrap();
}

pub fn write_to_file(text: &str, file_path: &str) -> std::io::Result<()> {
    fs::write(file_path, text)
}


fn read_json() -> Result<CV, Box<dyn Error>> {
    let file = File::open("data/cv.json")?;
    let reader = BufReader::new(file);

    let cv: CV = serde_json::from_reader(reader)?;

    dbg!(&cv);

    Ok(cv)
}