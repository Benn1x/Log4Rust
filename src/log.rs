use crate::date;
use std::fs::OpenOptions;
use std::io::Write;
use crate::load_log::Formatting;

pub enum Type{
    Terminal(String),
    Log(String),
    Bothe(String),
}

pub enum Log{
    Info,
    Warning,
    Error,
}

pub fn log(logs: Log, types: Type){
    match types{
        Type::Terminal(string) => {
            print_log(string, logs);
        }
        Type::Log(string) =>{
            write_log( string.format());
        }
        Type::Bothe(string) =>{
            print_log( string.clone(), logs);
            write_log(string.format());
        }
    }
}
pub fn write_log(logMsg: String){
    match  OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .read(true)
        .open("log.log")
        {
        Ok(mut e) =>{
            match e.write_all(logMsg.as_bytes()){
                Ok(_) =>{
                    e.flush().unwrap();
                },
                Err(e) => {
                    println!("\x1b[31m{}: Error with Wright to File: {}\x1b[0m", date::date(), e);
                },
            }
        }
        Err(e) =>{
            println!("\x1b[31m{}: {}\x1b[0m", date::date(), e);
        }
    }
}

pub fn print_log(string: String, logs: Log){
    match logs {
        Log::Info => println!("{}: {}\n", date::date(), string),
        Log::Warning => println!("\x1b[33m{}: {}\x1b[0m\n", date::date(), string),
        Log::Error => println!("\x1b[31m{}: {}\x1b[0m\n", date::date(), string),
    }
}
