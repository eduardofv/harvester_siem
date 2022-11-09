use reqwest::blocking::Client;
use serde_json::{Map, Value, json};
use std::{fs, thread, time};
use chrono;


pub fn get_siem(client: &Client, url: String) -> Result<Value, reqwest::Error> {
    let response = client.get(url).send()?;
    let value = response.json::<Value>();

    value
}

fn get_business_detail(client: &Client, id: &String) -> Result<Value, reqwest::Error> {
    let url = format!("https://siem.economia.gob.mx/detalle-establecimiento?id={}",
                      id);

    let detail = get_siem(client, url)?;

    Ok(detail)
}

fn get_business_profile(client: &Client, id: &String) -> Result<Value, reqwest::Error> {
    let url = format!("https://siem.economia.gob.mx/detalle-establecimiento-perfil?id={}",
                      id);

    let detail = get_siem(client, url)?;

    Ok(detail)
}

fn get_business_location(client: &Client, id: &String) -> Result<Value, reqwest::Error> {
    let url = format!("https://siem.economia.gob.mx/detalle-establecimiento-ubicacion?id={}",
                      id);

    let detail = get_siem(client, url)?;

    Ok(detail)
}

fn get_business_products(client: &Client, id: &String) -> Result<Value, reqwest::Error> {
    let url = format!("https://siem.economia.gob.mx/establecimientoproducto-por-establecimiento?idEstablecimiento={}",
                      id);

    let detail = get_siem(client, url)?;

    Ok(detail)
}


fn get_business_complement(client: &Client, id: &String) -> Result<Value, reqwest::Error> {
    let url = format!("https://siem.economia.gob.mx/establecimiento-complemento?id={}",
                      id);

    let detail = get_siem(client, url)?;

    Ok(detail)
}

fn get_business_countries(client: &Client, id: &String) -> Result<Value, reqwest::Error> {
    let url = format!("https://siem.economia.gob.mx/establecimiento-pais-por-establecimiento?id={}",
                      id);

    let detail = get_siem(client, url)?;

    Ok(detail)
}

pub fn get_business(client: &Client, id: String) -> Map<String, Value> {
    let detail = get_business_detail(client, &id).unwrap_or_else(|error| {
        eprintln!("Error reading detail for id={id}");
        json!(null)
    });

    let profile = get_business_profile(client, &id).unwrap_or_else(|error| {
        eprintln!("Error reading profile for id={id}");
        json!(null)
    });

    let location = get_business_location(client, &id).unwrap_or_else(|error| {
        eprintln!("Error reading location for id={id}");
        json!(null)
    });

    let products = get_business_products(client, &id).unwrap_or_else(|error| {
        eprintln!("Error reading products for id={id}");
        json!(null)
    });

    let complement = get_business_complement(client, &id).unwrap_or_else(|error| {
        eprintln!("Error reading complement for id={id}");
        json!(null)
    });

    let countries = get_business_countries(client, &id).unwrap_or_else(|error| {
        eprintln!("Error reading countries for id={id}");
        json!(null)
    });

    let mut business = Map::new();
    business.insert("detail".to_string(), detail);
    business.insert("profile".to_string(), profile);
    business.insert("location".to_string(), location);
    business.insert("products".to_string(), products);
    business.insert("complement".to_string(), complement);
    business.insert("countries".to_string(), countries);

    business 
}
