use reqwest::blocking::Client;
use serde_json::{Map, Value, json};
use std::collections::HashMap;
use std::{fs, thread, time};
use chrono;


fn get_serp_page(client: &Client, page: usize) -> Result<Value, reqwest::Error>  {
    
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

fn save_serp_list(list: Vec<Value>) -> std::io::Result<()> {
    let text = json!(list).to_string();
    let fname = format!("data/siem-serp-list.json");
    fs::write(fname, text)?;
    Ok(())
}

pub fn get_serp_full_list(client: &Client) -> Value {

    let mut list = get_serp_page(&client, 1)
        .expect("No se pudo leer la pagina 1");

    let max_page_index = list["pages"].as_u64().unwrap();
    let full :&mut Vec<Value> = list["list"].as_array_mut().unwrap();

    let mut page_index: u64 = 2;
    while page_index < max_page_index {
        let mut next_page = get_serp_page(&client, page_index as usize);
        match next_page {
            Ok(mut current) => {
                let next_page_list = &mut current["list"]; 
                if let Some(next_page_list_ref) = next_page_list.as_array_mut() {
                    full.append(next_page_list_ref);
                    if let Err(_) = save_serp_list(full.to_vec()) {
                        eprintln!("Error saving full list at page {page_index}");
                    }
                } else {
                    eprintln!("Could not get mut ref to list for page {page_index}");
                }
            },
            Err(err) => eprintln!("Could not get page {page_index}. Error: {err}"),
        }
        page_index += 1;
        if page_index % 10 == 0 {
            println!("{}\tINFO {page_index} serp pages scraped", chrono::offset::Local::now());
        }
        //courtesy delay
        thread::sleep(time::Duration::from_millis(100));
    }

    println!("full serp list len: {:?}", full.len());

    list

    //Result(Ok(first));

//    for page in (2..
}

