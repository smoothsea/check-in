use chrono::{DateTime, Local};
use crate::checkin::{CheckIn};

pub fn info<T:std::fmt::Display>(check: Box<& dyn CheckIn>, message: T)
{
    let datetime: DateTime<Local> = Local::now();
    let current_time = datetime.format("%Y-%m-%d %H:%M:%S").to_string();

    println!("{}:  {}:{}", current_time, check.name(), message);
}