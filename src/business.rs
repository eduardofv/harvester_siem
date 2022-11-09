use reqwest::blocking::Client;
use serde_json::{Map, Value, json};
use std::{fs, thread, time};
use chrono;

pub fn get_business(id: String) -> Result<Value, reqwest::Error> {
    let url = format!("https://siem.economia.gob.mx/detalle-establecimiento?id={}",
                      id);

    let response = client.get(url).send()?;

    let value = response.json::<Value>();

    println!("{:?}", value);

    value

}
