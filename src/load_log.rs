use std::fs;
use std::fs::read_to_string;
use std::io::Write;
use std::process::exit;
use serde_derive::Deserialize;
use crate::date;
use crate::date::{year, month, day, hour, second, minute};

#[derive(Deserialize)]
pub struct Config{
    application_version: String,
    application_name: String,
    formatting: String,
}

impl Config {
    pub fn new()->Self{
        let filename = "config.toml";
        let contents = match read_to_string(filename) {
            // If successful return the files text as `contents`.
            // `c` is a local variable.
            Ok(c) => c,
            // Handle the `error` case.
            Err(_) =>{
                let mut file= fs::File::create(filename).unwrap();
                file.write(b"application_version = '*'
application_name = ''
formatting = 'y/m/d h:M:s l'").unwrap();
                "application_version = '*'
application_name = ''
formatting = 'y/m/d h:M:s l'".to_string()

            }
        };
        let data: Config = match toml::from_str(&contents) {
            // If successful, return data as `Data` struct.
            // `d` is a local variable.
            Ok(d) => d,
            // Handle the `error` case.
            Err(_) => {
                // Write `msg` to `stderr`.
                eprintln!("Unable to load data from `{}`", filename);
                // Exit the program with exit code `1`.
                exit(1);
            }
        };
        Self{application_name: data.application_name, application_version: data.application_version, formatting: data.formatting}
    }
}


pub trait Formatting {
    fn format(&self)->String;
}

impl Formatting for String{
    fn format(&self)-> String{
        let format = Config::new().formatting;
        let l = self.clone();
        let formatted: String = format.chars().map(|x| match x {
            'y' => year(),
            'm' => month(),
            'd' => day(),
            'h' => hour(),
            'M' => minute(),
            's' => second(),
            'l' => l.clone(),
            _ => x.to_string(),
        }).collect();
        format!("{}\n",formatted)

    }
}