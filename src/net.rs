// Module uses reqwest as the underlying http client
extern crate reqwest;

// We use serde_json to work with Json
extern crate serde_json;

use ::std;

// Using error_chain for this module's errors
// Do not generate any "Result"-type, declare that our 
// error should be converitible from reqwest network errors,
// io errors or serde_json parse errors.
error_chain! {
    types {
        Error, ErrorKind, ResultExt;
    }

    foreign_links {
        Get(reqwest::Error);
        Read(std::io::Error);
        Parse(serde_json::error::Error);
    }
}

// Small wrapper around request::get, to perform a GET
pub fn get(url: &str) -> Result<String, self::Error> {
    use std::io::Read;

    let mut resp = reqwest::get(url)?;
    // Assert only in debug mode, what should we do here?
    debug_assert!(resp.status().is_success());

    let mut content = String::new();
    resp.read_to_string(&mut content)?;

    return Ok(content);
}

// Make a request to the jenkins json api, returns
// a serde_json object if everything went ok.
pub fn api(base: &[&str]) -> Result<serde_json::Value, self::Error> {
    let api_url = {
        let mut result = Vec::with_capacity(base.len() + 1);
        result.extend(base);
        result.push("api/json");
        result.join("/")
    };
    let json_string = self::get(&api_url)?;
    let json_obj: serde_json::Value = serde_json::from_str(&json_string)?;
    return Ok(json_obj);
}