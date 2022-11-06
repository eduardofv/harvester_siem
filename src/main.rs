use reqwest::blocking::Client;
use std::collections::HashMap;


fn main() -> Result<(), reqwest::Error> {
    let client = Client::new();

    let response = client.get(
        "https://httpbin.org"
    ).send()?;
    println!("{:?}",response);
    Ok(())
}
