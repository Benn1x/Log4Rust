use chrono::prelude::*;

pub fn date()-> String{

    format!("{}",chrono::Local::now().format("%d/%m/%Y %H:%M:%S"))
}

pub fn year() -> String{
    chrono::Local::now().year().to_string()
}

pub fn month() -> String{
    chrono::Local::now().month().to_string()
}

pub fn day()-> String{
    chrono::Local::now().day().to_string()
}

pub fn hour() -> String{
    chrono::Local::now().hour().to_string()
}

pub fn minute() -> String{
    chrono::Local::now().minute().to_string()
}

pub fn second() -> String{
    chrono::Local::now().second().to_string()
}

