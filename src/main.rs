mod checkin;
extern crate chrono;

use crate::checkin::{Smzdm, CheckIn};
use std::fs;
use serde::{Serialize, Deserialize};
use serde_json::Result;
use chrono::{DateTime, Local};
use std::{thread, time};

#[derive(Serialize, Deserialize, Debug)]
struct Config {
    smzdm: Option<String>,
    check_in_time: Option<String>,
}

fn main() {
    let mut args = std::env::args();
    args.next().unwrap();

    let mut file = "".to_string();
    match args.next() {
        Some(f) => file = f,
        None => file = "/etc/checkin.json".to_string(),
    }

    let interval = time::Duration::from_millis(60000);

    loop {
        let content = fs::read_to_string(file.clone()).expect("读取文件错误");
        let config: Config = serde_json::from_str(&content).expect("配置文件解析错误");
        let datetime: DateTime<Local> = Local::now();
        let current_time = datetime.format("%R").to_string();
        let mut check_in_time = "5:00".to_string();
        if let Some(time) = config.check_in_time {
            check_in_time = time;
        }

        if (check_in_time == current_time) {
            if let Some(smzdm) = config.smzdm {
                let smzdm = Smzdm::new(smzdm);
                match smzdm.check_in() {
                    Ok(s) => println!("签到成功"),
                    Err(e) => println!("签到错误：{}", e),
                }
            }
        }

        thread::sleep(interval);
    }
}