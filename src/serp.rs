use reqwest::blocking::Client;
use serde_json::{Value, json};
use std::{fs, thread, time};

pub fn load_serp_list() -> Vec<Value> {
   let text = fs::read_to_string("data/siem-serp-list.json")
        .expect("Could not read data/siem-serp-list.json");
    let list_json: Value = serde_json::from_str(&text)
        .expect("Could not parse json for data/siem-serp-list.json");
    //let list = list_json.as_object()
    //    .expect("Error json.as_object").clone();
    //println!("{:?}", list_json.as_array());

    //vec![]
    (&list_json.as_array().unwrap()).to_vec()
}

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

pub fn get_serp_full_list(client: &Client, starting_page: u64, courtesy_delay: u64) 
    -> Vec<Value> {

    let mut list = get_serp_page(&client, 1)
        .expect("No se pudo leer la pagina 1");

    let max_page_index = list["pages"].as_u64().unwrap();
    info!("There are {max_page_index} serp pages");
    let full :&mut Vec<Value> = list["list"].as_array_mut().unwrap();

    let mut page_index: u64 = starting_page;
    info!("Starting from page {starting_page} (page 1 is always added)");
    while page_index < max_page_index {
        let next_page = get_serp_page(&client, page_index as usize);
        match next_page {
            Ok(mut current) => {
                let next_page_list = &mut current["list"]; 
                if let Some(next_page_list_ref) = next_page_list.as_array_mut() {
                    full.append(next_page_list_ref);
                    if let Err(_) = save_serp_list(full.to_vec()) {
                        error!("Error saving full list at page {page_index}");
                    }
                } else {
                    error!("Could not get mut ref to list for page {page_index}");
                }
            },
            Err(err) => error!("Could not get page {page_index}. Error: {err}"),
        }
        page_index += 1;
        if page_index % 10 == 0 {
            info!("{page_index} serp pages scraped");
        }
        //courtesy delay
        thread::sleep(time::Duration::from_millis(courtesy_delay));
    }

    debug!("full serp list len: {:?}", full.len());

    full.to_vec()
}

