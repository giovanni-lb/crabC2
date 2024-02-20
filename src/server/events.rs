use rusqlite::{Connection};
use serde::Serialize;
use std::fs::File;
use std::io::prelude::*;

extern crate chrono;
use chrono::prelude::*;

use crate::db::Database;

#[derive(Serialize)]
pub struct Events{
    event_id: usize,
    description: String,
    time: String,
    ip: Option<String>,
    hostname: Option<String>,
    payload_name: Option<String>,
    id: Option<usize>,
}

impl Events{

    // This create a new event log, by initialiting the Events struct and populated with information about the current event
    pub fn new(event_id: usize, ip: Option<String>, hostname: Option<String>, payload_name: Option<String>, id: Option<usize> ) -> Events{
        let descriptions: [&str; 21] = [
            "New infected PC detected",
            "Connection opened from an infected PC",
            "Connection closed from an infected PC",
            "User connected",
            "User disconnected",
            "Stage 1 (Dropper) requested",
            "Stage 2 (Payload) requested",
            "Payload succeffuly installed",
            "Scheduled task started",
            "Scheduled task ended",
            "External discovery attempt of the C2",
            "Command success",
            "Command fail",
            "Exfiltration succes",
            "Exfiltration fail",
            "Unauthorize connection attempt",
            "C2 config changed",
            "Unautorized action from user",
            "C2 started",
            "C2 stopped",
            "Payload deleted"

        ];

        let description = if event_id > 0 && event_id <= descriptions.len() {
            descriptions[event_id - 1].to_string()
        } else {
            "Unknown event id".to_string()
        };
        // Get actual time and parse it
        let now = Utc::now();
        let time = now.format("%d-%m-%Y %H:%M").to_string();

        let db = Database::new("crabe.db").unwrap();

        let _ = db.add_new_event(event_id as i32, &description);

        Events{
            event_id,
            description,
            time,
            ip,
            hostname,
            payload_name,
            id,
        }


    }

    // this function is used in order to log the events in the logs/logs.txt file
    pub fn log_to_file(&self) -> Result<(), std::io::Error> {
        let path = "logs/logs.txt";

        // Create the file if not created and/or append to it 
        let mut file = File::options()
            .write(true)
            .append(true)
            .create(true)
            .open(path)?;
        
        let default_hostname = String::from("No Hostname");
        let default_ip = String::from("No IP");
        let default_payload_name = String::from("No Payload Name");

        let vhostname = self.hostname.as_ref().unwrap_or(&default_hostname);
        let vip = self.ip.as_ref().unwrap_or(&default_ip);
        let vpayload_name = self.payload_name.as_ref().unwrap_or(&default_payload_name);

        // format the event log 
        let new_log = match self.event_id {
            1 | 2 | 3 => format!("{} - {} | ip: {}, hostname : {} ", self.time, self.description, vip, vhostname),
            6 | 7 => format!("{} - {} | ip: {}, payload_name: {} ", self.time, self.description, vip, vpayload_name),
            8 | 21 => format!("{} - {} | payload_name: {} ", self.time, self.description, vpayload_name),
            19 => format!("{} - {} with ip: {}", self.time, self.description, vip),
            _ => format!("{} - {} ", self.time, self.description),

        };
        // write to the debug console
        println!("{}", new_log);
        // write to file
        writeln!(file, "{}", new_log)?;
    
        Ok(())
    }

    pub fn fetch_all(conn: &Connection) -> rusqlite::Result<Vec<Events>> {
        let mut stmt = conn.prepare("SELECT id, event_id, description, time FROM events")?;
        
        let events_rows = stmt.query_map([], |row| {
            Ok(Events {

                event_id: row.get(1)?,
                description: row.get(2)?,
                time: row.get(3)?,
                ip: None,
                hostname: None,
                payload_name: None,
                id: row.get(0)?
            })
        })?;

        let mut events = Vec::new();
        for events_row in events_rows {
            events.push(events_row?);
        }

        Ok(events)
    }

    pub fn count_all(conn: &Connection) -> rusqlite::Result<String>
    {
        let mut stmt = conn.prepare("SELECT COUNT(*) FROM events")?;
        let count: i32 = stmt.query_row([], |row| row.get(0))?;
        Ok(count.to_string())
    }


}
