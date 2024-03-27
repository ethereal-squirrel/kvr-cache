use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use std::collections::HashMap;
use std::env;
use std::sync::{Arc, Mutex};

struct KeyValueStore {
    store: Mutex<HashMap<String, String>>,
}

impl KeyValueStore {
    fn new() -> KeyValueStore {
        KeyValueStore {
            store: Mutex::new(HashMap::new()),
        }
    }

    fn set(&self, key: String, value: String) {
        let mut store = self.store.lock().unwrap();
        store.insert(key, value);
    }

    fn get(&self, key: String) -> Option<String> {
        let store = self.store.lock().unwrap();
        store.get(&key).cloned()
    }
}

async fn set_value(
    data: web::Data<Arc<KeyValueStore>>,
    info: web::Json<(String, String)>,
) -> impl Responder {
    let (key, value) = info.into_inner();
    data.set(key, value);
    HttpResponse::Ok().body("Value set successfully.")
}

async fn get_value(data: web::Data<Arc<KeyValueStore>>, key: web::Path<String>) -> impl Responder {
    match data.get(key.into_inner()) {
        Some(value) => HttpResponse::Ok().body(value),
        None => HttpResponse::NotFound().body("Key not found."),
    }
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let store = Arc::new(KeyValueStore::new());

    let args: Vec<String> = env::args().collect();
    let bind_address = args.get(1).expect("Usage: kvr-cache <bind_address>").as_str();

    println!("KVR Cache is running on: {}", bind_address);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(store.clone()))
            .route("/set", web::post().to(set_value))
            .route("/get/{key}", web::get().to(get_value))
    })
    .bind(bind_address)?
    .run()
    .await
}
