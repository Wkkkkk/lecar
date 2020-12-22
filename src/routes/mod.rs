use lecar::controller::Controller;
use rocket::State;
use std::sync::{Arc, Mutex};
use crate::routes::types::LecarData;
use rocket_contrib::json::Json;
use rocket::response::status::NotFound;

mod types;

#[get("/health")]
fn health_check() -> &'static str {
    "LeCaR is healthy running on Rocket!"
}

#[get("/cache/<key>")]
fn get_data(cache_controller_state: State<Arc<Mutex<Controller>>>, key: usize) -> Result<Json<LecarData>, NotFound<&'static str>> {
    let cloned_state_arc = Arc::clone(&cache_controller_state);
    let mut cache_controller = cloned_state_arc.lock().unwrap();

    match (*cache_controller).get(key) {
        Some(item) => Ok(Json(LecarData { key, value: item })),
        None => Err(NotFound("No such key!"))
    }
}

#[post("/cache", data = "<data>")]
fn insert_data(cache_controller_state: State<Arc<Mutex<Controller>>>, data: Json<LecarData>) {
    let cloned_state_arc = Arc::clone(&cache_controller_state);
    let mut cache_controller = cloned_state_arc.lock().unwrap();

    cache_controller.insert(data.key, data.value.clone());
}

pub fn launch_server() {
    let cache_controller = Arc::new(Mutex::new(Controller::new(2_000, 200, 200)));

    rocket::ignite()
        .manage(cache_controller)
        .mount("/", routes![
            health_check,
            get_data,
            insert_data
        ])
        .launch();
}
