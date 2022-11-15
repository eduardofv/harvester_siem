#[macro_use]
extern crate log;

use env_logger::Env;

use reqwest::blocking::Client;
//use serde_json::{Value};
use std::{fs, thread, time};
use std::collections::HashMap;


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
                    error!("Error with a file entry reading data/establecimientos");
                }
            }
            scraped_biz
        },
        Err(_) => { 
            HashMap::<String, usize>::new()
        }
    };

    for biz_id in selected {
        let id = String::from(biz_id["id"].as_str().unwrap());
        let filename = format!("{id}.json");
        if let Some(_) = scraped_biz.get(&filename) {
            info!("Already exists {}", &id);
        } else {
            info!("Scraping {}", &id);
            get_and_save_business(&client, &id, false);
            scraped_biz.insert(filename, 1);
            thread::sleep(time::Duration::from_millis(33));
        }
    }

    Some(())
}


fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or("INFO")).init();
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
    
    
    get_serp_full_list(&client, 2, 30);
    

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
