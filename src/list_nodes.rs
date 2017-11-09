extern crate itertools;

use ::net;
use ::config;

use ::std;

// Import and alias
use serde_json;
use serde_json::Value as Json;

#[derive(Debug, Deserialize)]
struct Computer {
    #[serde(rename="displayName")]
    name: String,
    #[serde(rename="numExecutors")]
    executors: u16,
    #[serde(rename="offlineCauseReason")]
    offline: Option<String>,
}

impl std::fmt::Display for Computer {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} ({})", self.name, self.executors)?;
        if let Some(ref offline_because) = self.offline {
            if offline_because.len() > 0 {
                write!(f, " \"{}\"", offline_because)?;
            }
        }
        Ok(())
    }
}

fn list_computers<'api>(api_response: &'api Json) -> Vec<Computer> {
    return serde_json::from_value(api_response["computer"].clone()).expect("Could not deserialize computer array");
}

// Function for the "list nodes" command
pub fn execute(config: &config::Config) {
    // Declare that we want to use the `Itertools` trait
    use self::itertools::Itertools;

    // Nested functions are also cool
    fn is_some_and_not_empty(c: &Computer) -> bool {
        // We can do this with Option::filter in newer Rust versions
        c.offline.as_ref().map_or(false, |s| s.len() > 0)
    }

    match net::api(&[&config.jenkins.server, "computer"]) {
        Ok(ref response) => { // Use of `ref` keyword to capture by refence, cause the type to be &Json
            println!("Listing jobs:");
            // Must sort the computers to get our nodes in order, we can use some built in functions
            // To sort on the `offline` field
            let mut computers = list_computers(response);
            computers.sort_by(|a, b| Ord::cmp(&a.offline, &b.offline));

            for (offline, computers) in &computers.into_iter().group_by(is_some_and_not_empty) {
                // We don't need ternary operators, because "if" is an expression
                println!("{}:", if offline { "offline" } else { "online" });
                for comp in computers {
                    println!("  {}", comp);
                }
            }
        },
        Err(err) => println!("Could not fetch computer list! ({:?})", err),
    }
}