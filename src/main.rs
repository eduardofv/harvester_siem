use reqwest::blocking::Client;
use serde_json::{Map, Value, json};
use std::{fs, thread, time};
use std::collections::HashMap;
use chrono;

use crate::catalogs::*;
pub mod catalogs;

use crate::serp::*;
pub mod serp;

use crate::business::*;
pub mod business;


fn scrap_businesses(client: &Client) -> Option<()> {
    let serp_list = load_serp_list();
    let selected = &serp_list;//[0..100];

    let files = fs::read_dir("data/establecimientos/");
    let mut scraped_biz = match files {
        Ok(files) => {
            let mut scraped_biz = HashMap::new();
            for file in files {
                if let Ok(file) = file {
                    scraped_biz.insert(file.file_name().into_string().unwrap(), 1usize);
                } else {
                    eprintln!("Error with a file entry reading data/establecimientos");
                }
            }
            scraped_biz
        },
        Error => { 
            HashMap::<String, usize>::new()
        }
    };

    for biz_id in selected {
        let id = String::from(biz_id["id"].as_str().unwrap());
        let filename = format!("{id}.json");
        if let Some(_) = scraped_biz.get(&filename) {
            println!("{}\tINFO Already exists {}", chrono::offset::Local::now(), &id);
        } else {
            println!("{}\tINFO Scraping {}", chrono::offset::Local::now(), &id);
            get_and_save_business(&client, &id, false);
            scraped_biz.insert(filename, 1);
            thread::sleep(time::Duration::from_millis(33));
        }
        /*
        let biz = get_business(&client, &id);
        save_business(&id, &biz).unwrap_or_else(|error| {
            eprintln!("{}\rERROR {} saving {}", 
                      chrono::offset::Local::now(),
                      error, 
                      &id);
        });
        */
    }

    Some(())
}


fn main() {
    let client = Client::new();

    //let catdef = load_catalog_definition();

    //scrap_businesses(&client);

    //let id = String::from("4039");
    //let detail = get_business(&client, &id);
    //save_business(&id, &detail).unwrap_or_else(|error| {
    //    eprintln!("Error {} saving {}", error, &id);
    //});
    //println!("{:?}", detail);

    //let res = load_serp_list();
    //println!("{:?}", res);
    
    
    let res = get_serp_full_list(&client, 2, 30);
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
