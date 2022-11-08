use reqwest::blocking::Client;
use serde_json::{Map, Value, json};
use std::collections::HashMap;
use std::fs;

pub mod catalogs;
use crate::catalogs::*;


fn main() {
    let client = Client::new();

    let catdef = load_catalog_definition();

    //let catalogs = get_and_save_catalogs(&client, catdef)
    //    .expect("Error in get_and_save catalogs");
    //let catalogs = load_catalogs(catdef)
    //    .expect("Error loading catalogs");

    let municipios = load_municipios();
    println!("{}", municipios.unwrap().len());
    //let municipios = get_and_save_municipios(&client, &catalogs["estados"]);

    //for i in catalogs["estados"].as_array().unwrap() {
    //    println!("{:?}", i["descripcion"]);
    //}


    /*******
    let result = sandbox(client);
    println!("{:?}", result);
    */
}

fn sandbox(client: Client) -> Result<(), reqwest::Error> {
    //detalle establecimiento
    //let url = "https://siem.economia.gob.mx/detalle-establecimiento?id=1739";

    //catalogo estados
    let url = "https://siem.economia.gob.mx/municipios-x-edo?idEntidadFederativa=1";

    //404
    //let url = "https://siem.economia.gob.mx/Mmunicipios-x-edo?idEntidadFederativa=1";

    let response = client.get(url).send()?;
    println!("\nResponse: {:?}", response);

    let text = response.text().unwrap();
    println!("\nText: {:?}", text);

    let obj: Value = serde_json::from_str(&text).expect("JSON not well formed");
    println!("\nJSON Object: {:?}", obj);

    let arr = obj.as_array().unwrap();
    println!("\nArray?: {:?}", arr);

    /*
    let v: Vec<Value> = serde_json::from_str(&text).unwrap();
    
    for val in &v {
        println!("{}", val);
    }
    */


    Ok(())
}
