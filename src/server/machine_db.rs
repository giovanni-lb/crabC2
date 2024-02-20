use rusqlite::{Connection};
use serde::Serialize;
use serde::Deserialize;

#[derive(Serialize)]
pub struct InfectedMachine {
    id: i32,
    machine_name: String,
    ip: String,
    mac: String,
    users: String,
    os: String,
    os_version: String,
    first_active: String,
    last_active: String,
    payload_id: i32,
}

#[derive(Deserialize, Serialize)]
pub struct MachineInfo {
    pub hostname: String,
    pub mac: String,
    pub users: String, 
    pub os: String,
    pub os_version: String,
}

impl InfectedMachine {
    pub fn fetch_all(conn: &Connection) -> rusqlite::Result<Vec<InfectedMachine>> {
        let mut stmt = conn.prepare("SELECT id, machine_name, ip, mac, users, os, os_version, first_active, last_active, payload_id FROM infected_machine")?;
        
        let machine_rows = stmt.query_map([], |row| {
            Ok(InfectedMachine {
                id: row.get(0)?,
                machine_name: row.get(1)?,
                ip: row.get(2)?,
                mac: row.get(3)?,
                users: row.get(4)?,
                os: row.get(5)?,
                os_version: row.get(6)?,
                first_active: row.get(7)?,
                last_active: row.get(8)?,
                payload_id: row.get(9)?,
            })
        })?;
        let mut machines = Vec::new();
        for machine_row in machine_rows {
            machines.push(machine_row?);
        }

        Ok(machines)
    }

    pub fn count_all(conn: &Connection) -> rusqlite::Result<String>
    {
        let mut stmt = conn.prepare("SELECT COUNT(*) FROM infected_machine")?;
        let count: i32 = stmt.query_row([], |row| row.get(0))?;
        Ok(count.to_string())
    }

    pub fn fetch_machine_by_id(conn: &Connection, id: u64) -> rusqlite::Result<InfectedMachine> {
        let query = "SELECT id, machine_name, ip, mac, users, os, os_version, first_active, last_active, payload_id FROM infected_machine WHERE id = ?";

        conn.query_row(query, [id], |row| {
            Ok(InfectedMachine {
                id: row.get(0)?,
                machine_name: row.get(1)?,
                ip: row.get(2)?,
                mac: row.get(3)?,
                users: row.get(4)?,
                os: row.get(5)?,
                os_version: row.get(6)?,
                first_active: row.get(7)?,
                last_active: row.get(8)?,
                payload_id: row.get(9)?,
            })
        })
    }

    pub fn count_by_os(conn: &Connection) -> rusqlite::Result<Vec<(String, u64)>> {
        let mut stmt = conn.prepare("SELECT os, COUNT(*) as count FROM infected_machine GROUP BY os")?;
        let os_type = stmt.query_map([], |row| {
            Ok((row.get(0)?, row.get(1)?))
        })?;

        let mut os_all = Vec::new();

        for os in os_type 
        {
            os_all.push(os?);
        }
    
        Ok(os_all)
    }
}
