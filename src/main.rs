use reqwest::blocking::Client;
use serde_json::{Map, Value, json};
use std::collections::HashMap;
use std::{fs, thread, time};
use chrono;

use crate::catalogs::*;
pub mod catalogs;

use crate::serp::*;
pub mod serp;

use crate::business::*;
pub mod business;

fn main() {
    let client = Client::new();

    //let catdef = load_catalog_definition();

    let serp_list = load_serp_list();

    let selected = &serp_list[0..100];

    for biz_id in selected {
        let id = String::from(biz_id["id"].as_str().unwrap());
        println!("{}\tINFO Scraping {}", chrono::offset::Local::now(), &id);
        get_and_save_business(&client, &id, true);
        /*
        let biz = get_business(&client, &id);
        save_business(&id, &biz).unwrap_or_else(|error| {
            eprintln!("{}\rERROR {} saving {}", 
                      chrono::offset::Local::now(),
                      error, 
                      &id);
        });
        */
        //Courtesy delay
        thread::sleep(time::Duration::from_millis(33));
    }

    //let id = String::from("4039");
    //let detail = get_business(&client, &id);
    //save_business(&id, &detail).unwrap_or_else(|error| {
    //    eprintln!("Error {} saving {}", error, &id);
    //});
    //println!("{:?}", detail);

    //let res = load_serp_list();
    //println!("{:?}", res);
    //let res = get_serp_full_list(&client);
    //println!("{:?}", res);

    //let catalogs = get_and_save_catalogs(&client, catdef)
    //    .expect("Error in get_and_save catalogs");
    //let catalogs = load_catalogs(catdef)
    //    .expect("Error loading catalogs");

    //let municipios = load_municipios();
    //println!("{}", municipios.unwrap().len());
    //let municipios = get_and_save_municipios(&client, &catalogs[]);

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
