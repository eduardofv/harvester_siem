use reqwest::blocking::Client;
use serde_json::{Map, Value, json};
use std::fs;
use std::path::Path;


pub fn save_business(id: &String, data: &Map<String, Value>) -> std::io::Result<()> {
    let text = json!(data).to_string();
    let fname = format!("data/establecimientos/{id}.json");
    fs::write(fname, text)?;
    Ok(())
}

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
    let url = format!("https://siem.economia.gob.mx/establecimiento-ubicacion.json?id={}",
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

pub fn get_and_save_business(client: &Client, id: &String, ignore_if_exists: bool) {

    if ignore_if_exists {
        if Path::new(&format!("data/establecimientos/{id}.json")).exists() {
            error!("Already exists {id}");
            return;
        }
    }

    let biz = get_business(&client, &id);
    save_business(&id, &biz).unwrap_or_else(|error| {
        error!("Error {} saving {}", error, &id);
    });
}

pub fn get_business(client: &Client, id: &String) -> Map<String, Value> {
    let detail = get_business_detail(client, &id).unwrap_or_else(|error| {
        error!("Error reading detail for id={id}: {error}");
        json!(null)
    });

    let profile = get_business_profile(client, &id).unwrap_or_else(|error| {
        error!("Error reading profile for id={id}: {error}");
        json!(null)
    });

    let location = get_business_location(client, &id).unwrap_or_else(|error| {
        error!("Error reading location for id={id}: {error}");
        json!(null)
    });

    let products = get_business_products(client, &id).unwrap_or_else(|error| {
        error!("Error reading products for id={id}: {error}");
        json!(null)
    });

    let complement = get_business_complement(client, &id).unwrap_or_else(|error| {
        error!("Error reading complement for id={id}: {error}");
        json!(null)
    });

    let countries = get_business_countries(client, &id).unwrap_or_else(|error| {
        error!("Error reading countries for id={id}: {error}");
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
