use reqwest::blocking::Client;
use serde_json::{Map, Value, json};
use std::collections::HashMap;
use std::fs;


pub fn get_serp_page(client: &Client, page: usize) -> Result<Value, reqwest::Error>  {
    
    let url = format!("https://siem.economia.gob.mx/establecimientos-publicos-x-criterios?id=&catEntidadFederativaFk=0&catActividad=0&catCamaraFk=&nombreComercial=&importa=2&exporta=2&publico=2&catEdoEstablecimientoFk=0&pageNum={}&orderBy=&desc=0",
                      page);

    let response = client.get(url).send()?;
    /*
        .expect(&format!("Could not get serp page {}", page))
        .text()
        .expect(&format!("Could not get text from serp page {}", page));
    */

    let value = response.json::<Value>();

    value
}

pub fn get_serp_full_list(client: &Client) -> Value {

    let mut list = get_serp_page(&client, 1)
        .expect("No se pudo leer la pagina 1");

    let max_page = list["pages"].as_u64().unwrap();
    println!("max: {max_page}\nlist: {:?}", list["list"].as_array()); 

    let mut second = get_serp_page(&client, 2)
        .expect("Err");

    let full :&mut Vec<Value> = list["list"].as_array_mut().unwrap();
    full.append(&mut second["list"].as_array_mut().unwrap());

    println!("{:?}", full);

    list

    //Result(Ok(first));

//    for page in (2..
}

