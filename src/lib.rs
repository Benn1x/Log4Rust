use crate::load_log::Formatting;
use crate::log::{Log, print_log, Type, write_log};

mod date;
pub mod log;
pub mod load_log;

/// # Logger
/// # How to log?
/// You need to call the Fn() `log()` this Fn() takes to params
/// the first is the enum Log, with this enum you can specify if this should be an info,
/// a warning or an error, as the second parm you need a string
/// # Examples
///```rust
/// use Logging4Dummys as log;
/// fn main() -> (){
///     log::Log(log::Log::Info, "This is an Info".to_string());
///     // Some Code
/// }
/// ```
/// This example would Log The words "This is an Info" as an Info
/// # Config
/// The config is built up like this:
///```toml
///application_version = '' //Here comes the Version of the Current application in
/// application_name = '' //This is the name of the application
/// formatting = 'y/m/d h:M:s l'
/// //in this format y is the Year m is the Month d is the Day h is the Hour M is the Minute s is the Second and l is the logging Message
/// stdout = 'log'
/// //This defines where the Log is written to there are 3 Options: console, this logs the to the console, log, this logs into an log.log file, and both this
/// // logs into the console as well as into the log.log file
/// ```

pub fn log(logs: Log, log_msg: String){
    match log_msg.is_type(){
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

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
