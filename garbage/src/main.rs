

#![allow(unused)]
use log::{debug, LevelFilter};

///various log levels
#[derive(Clone, PartialEq, Debug)]
pub enum LogLevel{
    Info, 
    Warning,
    Error,
}

pub fn log(level: LogLevel, message: &str)-> String{
    let level = format!("{:?}", level);
    format!("[{}]: {}", level.to_uppercase(), message)

}

pub fn info(message: &str){

    log(LogLevel::Info, message);
    
        //println!("{}", message);
        println!("{}",log(LogLevel::Info, message))

}

fn main()
{
    info("Do not escape!");
}