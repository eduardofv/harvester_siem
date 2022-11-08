use reqwest::blocking::Client;
use serde_json::{Map, Value, json};
use std::collections::HashMap;
use std::fs;

pub fn load_catalog_definition() -> Map<String, Value> {
    let text = fs::read_to_string("catalog_definition.json")
        .expect("Could not read catalog_definition.json");
    let catdef_json: Value = serde_json::from_str(&text)
        .expect("Could not parse json for 'catalog_definition.json");
    let catdef = catdef_json.as_object()
        .expect("Error json.as_object").clone();
    //println!("{:?}", catdef);
    catdef
}

pub fn get_catalog(client: &Client, cat_name: &str, url: &str) -> Value {
    let response = client.get(url).send()
        .expect(&format!("Could not get catalog {}", cat_name))
        .text()
        .expect(&format!("Could not get text from catalog {}", cat_name));

    serde_json::from_str(&response)
        .expect(&format!("Could not parse JSON {}", response))
}

pub fn save_catalog(cat_name: &str, jsn: &Value) -> std::io::Result<()> {
    let text = jsn.to_string();
        //.expect(&format!("Could not serialize catalog {}", cat_name));
    let fname = format!("data/siem-catalogo-{}.json", cat_name);
    fs::write(fname, text)?;
    Ok(())
}

pub fn get_and_save_catalogs(client: &Client, catdef: Map<String, Value>) -> std::io::Result<HashMap<String, Value>> {
    let mut catalogs = HashMap::new();

    for (cat_name, url) in catdef {
        let url = url.as_str().unwrap();
        let cat_name_l = cat_name;//.clone();
        //println!("{}, {}", cat_name, url);
        let cat_json = get_catalog(&client, &cat_name_l, url);
        save_catalog(&cat_name_l, &cat_json)?;
        catalogs.insert(cat_name_l.to_owned(), cat_json);
    }

    Ok(catalogs)
}

pub fn load_catalogs(catdef: Map<String, Value>) -> std::io::Result<HashMap<String, Value>> {
    let mut catalogs = HashMap::new();

    for (cat_name, _) in catdef {
        let fname = format!("data/siem-catalogo-{}.json", cat_name);
        let cat_string = fs::read_to_string(fname)?;
        let cat_json = serde_json::from_str(&cat_string)
            .expect(&format!("Could nor parse json for {}", cat_string));
        catalogs.insert(cat_name.to_owned(), cat_json);
    }

    Ok(catalogs)
}

pub fn get_municipios(client: &Client, id_estado: u64) -> Value {
    //assert!(id_estado > 0 && id_estado < 33);

    let url = format!("https://siem.economia.gob.mx/municipios-x-edo?idEntidadFederativa={}", id_estado);
    println!("{}", url);
    
    let response = client.get(url).send()
        .expect(&format!("Could not get municipios {}", id_estado))
        .text()
        .expect(&format!("Could not get text from municipios {}", id_estado));

    serde_json::from_str(&response)
        .expect(&format!("Could not parse JSON {}", response))
}

//fn get_and_save_municipios(client: &Client, estados_json: &Value) -> std::io::Result<HashMap<u64, Value>> {
pub fn get_and_save_municipios(client: &Client, estados_json: &Value) -> std::io::Result<Map<String, Value>> {
    let mut municipios = Map::new();

    for estado in estados_json.as_array().unwrap() {
        println!("{:?}", estado);
        let id = estado["id"].as_u64().unwrap();
        let mun_estado = get_municipios(&client, id);
        let jid = id.to_string();
        municipios.insert(jid, mun_estado);
    }

    let text = json!(municipios).to_string();
    let fname = format!("data/siem-catalogo-municipios.json");
    fs::write(fname, text)?;

    println!("{}", municipios.len());
    Ok(municipios)
}


pub fn load_municipios() -> std::io::Result<Map<String, Value>> {
    
    let fname = format!("data/siem-catalogo-municipios.json");
    let cat_string = fs::read_to_string(fname)?;
    let municipios = serde_json::from_str(&cat_string)
            .expect(&format!("Could nor parse json for municipios"));

    Ok(municipios)
}

