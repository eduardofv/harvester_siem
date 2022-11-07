use reqwest::blocking::Client;
use serde_json::{Map, Value};
use std::fs;

struct Catalog {
    table_name: String,
}

fn load_catalog_definition() -> Map<String, Value> {
    let text = fs::read_to_string("catalog_definition.json")
        .expect("Could not read catalog_definition.json");
    let catdef_json: Value = serde_json::from_str(&text)
        .expect("Could not parse json for 'catalog_definition.json");
    let catdef = catdef_json.as_object()
        .expect("Error json.as_object").clone();
    println!("{:?}", catdef);
    catdef
}

fn get_catalog(client: &Client, cat_name: &str, url: &str) -> Value {
    let activo = true.to_string();
    let todos = true.to_string();
    let response = client.get(url).send()
        .expect(&format!("Could not get catalog {}", cat_name))
        .text()
        .expect(&format!("Could not get text from catalog {}", cat_name));

    serde_json::from_str(&response)
        .expect(&format!("Could not parse JSON {}", response))
}

fn main() {
    let client = Client::new();

    let catdef = load_catalog_definition();
    //println!("{:?}", catdef);

    for (cat_name, url) in catdef {
        let url = url.as_str().unwrap();
        let cat_name = cat_name.as_str();
        println!("{}, {}", cat_name, url);
        let cat = get_catalog(&client, cat_name, url);
        println!("{:?}", cat);
    }

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
