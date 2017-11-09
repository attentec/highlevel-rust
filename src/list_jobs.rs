use ::net;
use ::config;

// Import and alias
use serde_json::Value as Json;

// Lifetime annotations! They could be elided here, but 
// this demonstrates that it is safe to return a vector of
// references back into the original Json object. Rust
// guarantees that these pointers never dangle.
//fn list_jobs<'api>(api_response: &'api Json) -> Vec<&'api str> {
fn list_jobs<'api>(api_response: &'api Json) -> Vec<&'api str> {
    let raw_jobs = api_response["jobs"].as_array().expect("Jobs was not an array");
    let mut result = Vec::with_capacity(raw_jobs.len());
    for job in raw_jobs {
        result.push(job["name"].as_str().expect("job name was not a string"));
    }
    return result;
}

// Only compile this inline module when in the `test` configuration
#[cfg(test)]
mod test {

    // This function is a unittest
    #[test]
    fn list_jobs_extracts_jobs() {
        // Use of json! macro to create a json-value inline.
        let example_api_response = json!({
            "jobs": [
                {
                    "name": "alpha",
                },
                {
                    "name": "beta",
                }
            ]
        });

        // Access function in parent namespace with `super::`
        // Use vec! macro to create a Vec
        assert_eq!(super::list_jobs(&example_api_response), vec!["alpha", "beta"]);
    }
}

// Function for the "list jobs" command
pub fn execute(config: &config::Config) {
    match net::api(&[&config.jenkins.server]) {
        Ok(ref response) => { // Use of `ref` keyword to capture by refence, cause the type to be &Json
            println!("Listing jobs:");
            for job in list_jobs(response) {
                println!("  {}", job);
            }
        },
        Err(err) => println!("Could not fetch joblist! ({:?})", err),
    }
}