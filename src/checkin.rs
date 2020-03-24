use reqwest::*;
use std::io::{Read};
use reqwest::header::HeaderMap;
use regex::Regex;
use std::result::Result;
use std::error::Error;

pub struct Smzdm {
    cookie: String
}

pub trait CheckIn {
    fn check_in(&self) -> Result<String, Box<dyn Error>>;
}

impl Smzdm {
    pub fn new(cookie: String) -> Smzdm{
        Smzdm{
            cookie:cookie,
        }

    }
}

impl CheckIn for Smzdm {
    fn check_in(&self) -> Result<String, Box<dyn Error>> {
        let client = reqwest::Client::new();
        let mut headers = HeaderMap::new();
        headers.insert(header::COOKIE, self.cookie.parse().unwrap());
        headers.insert(header::REFERER, "https://www.smzdm.com/".to_string().parse().unwrap());

        let jsonpCallbackName = "jQuery112404811921179876988_1584931055407";
        let timestamp = "1584931055412";
        let url = format!("https://zhiyou.smzdm.com/user/checkin/jsonp_checkin?callback={}&_={}", jsonpCallbackName, timestamp);

        let mut res = client.get(& url)
            .headers(headers)
            .send()?;
        let mut body = "".to_string();

        res.read_to_string(& mut body)?;

        let reg = Regex::new("error_code\":0,").unwrap();
        match reg.find(& body) {
            Some(T) => return Ok(body),
            None => return Err(format!("签到返回：{}", body).into()),
        }
    }
}