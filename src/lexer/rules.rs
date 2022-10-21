use regex::Regex;
use std::{fs, str::FromStr};
use serde_json::{Result, Value};

pub struct Rules {
    rules: Vec<(String, Regex)>,
}

#[allow(dead_code)]
impl Rules {
    pub fn new() -> Rules{
        return Rules {rules: Vec::new()};
    }

    pub fn from_filepath(filepath: &str) -> Rules{
        let mut rules: Rules = Rules::new();
        let content: String = fs::read_to_string(filepath).expect("Should have been able to read;");
        let json_rules: Value = serde_json::from_str(&content).unwrap();
        for rule in json_rules.as_array().unwrap() {
            let token: String = String::from_str(rule["token"].as_str().unwrap()).unwrap();
            let regx: Regex = Regex::new(rule["pattern"].as_str().unwrap()).unwrap();
            println!("Loaded token: {} with rule: {}", token, regx.as_str());
            rules.rules.push((token, regx));
        }
        return rules;
    }

    pub fn get_rules(&self) -> Vec<(String, Regex)>{
        return self.rules.clone();
    }
}