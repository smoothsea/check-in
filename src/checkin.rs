use reqwest::*;
use std::io::{Read};
use reqwest::header::HeaderMap;
use regex::Regex;
use std::result::Result;
use std::error::Error;
use serde::{Serialize, Deserialize};

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

pub struct Tieba {
    cookie: String
}

#[derive(Serialize, Deserialize, Debug)]
struct TiebaItem {
    forum_name: String,
}

impl Tieba {
    pub fn new(cookie: String) -> Tieba {
        Tieba{
            cookie:cookie,
        }
    }
}

impl CheckIn for Tieba {
    fn check_in(&self) -> Result<String, Box<dyn Error>> {
        let client = reqwest::Client::new();
        let mut headers = HeaderMap::new();
        headers.insert(header::COOKIE, self.cookie.parse().unwrap());
        headers.insert(header::REFERER, "https://tieba.baidu.com/".to_string().parse().unwrap());

        let url = "https://tieba.baidu.com/";
        let signUrl = "https://tieba.baidu.com/sign/add";

        let mut res = client.get(url)
            .headers(headers.clone())
            .send()?;
        let mut body = "".to_string();

        res.read_to_string(& mut body)?;

        let reg = Regex::new("Module.use\\(\"spage/widget/AsideV2\",\\s*?\\[(.*?)\\]\\)").unwrap();
        let mut panel_body = "".to_string();
        match reg.captures_iter(& body).next() {
            Some(m) => panel_body = m[1].to_string(),
            None => return Err("解析错误".into()),
        }

        let mut list: Vec<TiebaItem> = vec![];
        match serde_json::from_str(& panel_body) {
            Ok(v) => list = v,
            Err(e) => return Err("登陆信息错误".into()),
        }

        for item in list.into_iter() {
            let params = [("ie", "utf8"), ("kw", & item.forum_name)];
            if let Ok(result) = client.post(signUrl).headers(headers.clone())
                .form(& params).send() {
            }
        }

        Ok("".to_string())
    }
}