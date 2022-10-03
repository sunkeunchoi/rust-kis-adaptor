#![allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Config {
    pub configuration: kis_api::apis::configuration::Configuration,
    pub appkey: String,
    pub appsecret: String,
    pub account: String,
    pub is_paper: bool,
    pub acnt_prdt_cd: String,
    pub cano: String,
}
impl Config {
    pub fn new(appkey: &str, appsecret: &str, account_number: &str, is_real: Option<bool>) -> Self {
        let (base_path, is_paper) = match is_real {
            Some(real) => {
                if real {
                    ("https://openapi.koreainvestment.com:9443", false)
                } else {
                    ("https://openapivts.koreainvestment.com:29443", true)
                }
            }
            None => ("https://openapivts.koreainvestment.com:29443", true),
        };
        let accounts: Vec<&str> = account_number.split('-').collect();
        let cano = Some(accounts[0])
            .unwrap_or_else(|| panic!("Invalid ACCOUNT_NUMBER"))
            .to_string();
        let acnt_prdt_cd = Some(accounts[1])
            .unwrap_or_else(|| panic!("Invalid ACCOUNT_NUMBER"))
            .to_string();
        let client = reqwest::Client::new();
        let configuration = kis_api::apis::configuration::Configuration {
            base_path: base_path.to_owned(),
            user_agent: None,
            client,
            basic_auth: None,
            oauth_access_token: None,
            bearer_access_token: None,
            api_key: None,
        };
        Config {
            configuration,
            appkey: appkey.to_owned(),
            appsecret: appsecret.to_owned(),
            account: account_number.to_owned(),
            is_paper,
            cano,
            acnt_prdt_cd,
        }
    }
}
