#[get("/")]
fn get_data() -> Vec<u8> {
    (*"Hello, World!".as_bytes()).to_vec()
}

pub fn launch_server() {
    rocket::ignite().mount("/", routes![get_data]).launch();
}
