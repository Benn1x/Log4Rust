use std::fs;
use std::fs::read_to_string;
use std::io::Write;
use std::process::exit;
use serde_derive::Deserialize;
use crate::log::Type;
use crate::date::{year, month, day, hour, second, minute};

///Into this struct the Config will be parsed
/// # Explanations
/// application_version: Is the current Version of the application
/// application_name: Is the name of the application
/// formatting: in this format y is the Year m is the Month d is the Day h is the Hour M is the Minute s is the Second and l is the logging Message
/// stdout: This defines where the Log is written to there are 3 Options: console, this logs the to the console, log, this logs into an log.log file, and both this
/// filename: This is the name of the log file, there is no need to add the log
/// logs into the console as well as into the log.log file
#[derive(Deserialize)]
pub struct Config{
    pub application_version: String,
    pub application_name: String,
    pub formatting: String,
    pub stdout: String,
    pub filename: String
}

impl Config {
    ///Read the Config and if it is not set it will create one
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
formatting = 'y/m/d h:M:s l'
stdout = 'both'
filename = 'log'").unwrap();
                "application_version = '*'
application_name = ''
formatting = 'y/m/d h:M:s l'
stdout = 'both'
filename = 'log'".to_string()

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
        Self{application_name: data.application_name, application_version: data.application_version, formatting: data.formatting, stdout: data.stdout, filename: data.filename}
    }
}


pub trait Formatting {
    fn format(&self)->String;
    fn is_type(&self) -> Type;
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
    fn is_type(&self) -> Type {
        let format = Config::new().stdout.to_ascii_lowercase();
        match format.as_str() {
            "console" => Type::Terminal(self.clone()),
            "log" => Type::Log(self.clone()),
            "both" => Type::Bothe(self.clone()),
            _ => Type::Log(self.clone())
        }
    }
}