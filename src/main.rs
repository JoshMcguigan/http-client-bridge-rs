#![feature(plugin, decl_macro)]
#![plugin(rocket_codegen)]

extern crate rocket;

use std::io::Read;
use std::io;
use std::sync::Mutex;
use std::collections::HashMap;
use std::path::PathBuf;
use rocket::State;
use rocket::Request;
use rocket::http::ContentType;
use rocket::data::FromData;
use rocket::Data;
use rocket::outcome::Outcome::Success;
use rocket::data::Outcome;
use rocket::outcome::Outcome::Failure;
use rocket::http::Status;
use rocket::response::Content;
use std::sync::Arc;
use rocket::Config;
use rocket::config::Environment;
use std::thread;
use std::time;

#[derive(Clone, Debug)]
struct SubmittedData {
    data: Vec<u8>,
    content_type: Option<ContentType>
}

impl FromData for SubmittedData {
    type Error = io::Error;

    fn from_data(req: &Request, data: Data) -> Outcome<Self, Self::Error> {
        let mut bytes = Vec::new();
        match data.open().read_to_end(&mut bytes) {
            Ok(_) => Success(
                SubmittedData {
                    data: bytes,
                    content_type: req.content_type().map(|x| x.to_owned())
                }),
            Err(e) => Failure((Status::InternalServerError,e))
        }
    }
}

#[derive(Default)]
struct AppData {
    data: Mutex<HashMap<String, SubmittedData>>,
}

#[get("/api/<path..>")]
fn read_data(app_data: State<Arc<AppData>>, path: PathBuf) -> Content<Option<Vec<u8>>> {
    let data = app_data.data.lock().unwrap();
    match data.get(path.to_str().unwrap()) {
        Some(submitted_data) => {
            let submitted_data = submitted_data.clone();
            Content(submitted_data.content_type.unwrap_or(ContentType::new("text", "plain")), Some(submitted_data.data))
        },
        None => Content(ContentType::new("text", "plain"), None)
    }
}

#[post("/<path..>", data="<post_data>")]
fn write_data(app_data: State<Arc<AppData>>, path: PathBuf, post_data: SubmittedData) {
    let mut data = app_data.data.lock().unwrap();
    data.insert(String::from(path.to_str().unwrap()), post_data);
}


fn main() {
    let write_app_state = Arc::new(AppData {..Default::default()});
    let read_app_state = write_app_state.clone();

    let write_config = Config::build(Environment::active().unwrap()).port(3000).unwrap();
    let read_config = Config::build(Environment::active().unwrap()).port(8000).unwrap();

    thread::spawn(   || {
        rocket::custom(write_config, false).mount("/", routes![write_data])
            .manage(write_app_state)
            .launch();
    });

    rocket::custom(read_config, false).mount("/", routes![read_data])
        .manage(read_app_state)
        .launch();

    // TODO log when server starts
    // TODO allow user to specify which ports to use
    // TODO allow static file serving on GET server
}
