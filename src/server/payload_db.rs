use rusqlite::{Connection};
use serde::Serialize;

#[derive(Serialize)]
pub struct Payload {
    pub payload_id: i32,
    pub payload_name: String,
    pub payload_type: String,
    pub payload_data: String,
}

impl Payload {
    pub fn fetch_all(conn: &Connection) -> rusqlite::Result<Vec<Payload>> {
        // prepare the query for the database in order to fetch all payload
	let mut stmt = conn.prepare("SELECT payload_id, payload_name, payload_type, payload_data FROM payloads")?;
        
	// make the query and map payload table content 
        let payload_rows = stmt.query_map([], |row| {
            Ok(Payload {
                payload_id: row.get(0)?,
                payload_name: row.get(1)?,
                payload_type: row.get(2)?,
                payload_data: row.get(3)?,
            })
        })?;
	// create a new vector for the payload list
        let mut payloads = Vec::new();
        for payload_row in payload_rows {
	    // push the payload to the vector
            payloads.push(payload_row?);
        }

        Ok(payloads)
    }

    pub fn count_all(conn: &Connection) -> rusqlite::Result<String>
    {
    	// prepare the query to count payloads
        let mut stmt = conn.prepare("SELECT COUNT(*) FROM payloads")?;
        // fetch the result and return it
	let count: i32 = stmt.query_row([], |row| row.get(0))?;
        Ok(count.to_string())
    }
    pub fn fetch_payload_by_id(conn: &Connection, id: u64) -> rusqlite::Result<Payload> {
        // prepare the query to fecth payload information for payload_id = id
	let query = "SELECT payload_id, payload_name, payload_type, payload_data FROM payloads WHERE payload_id = ?";

	// map the result 
        conn.query_row(query, [id], |row| {
            Ok(Payload {
                payload_id: row.get(0)?,
                payload_name: row.get(1)?,
                payload_type: row.get(2)?,
                payload_data: row.get(3)?,
            })
        })
    }

    pub fn delete_payload_by_id(conn: &Connection, id: u64) -> Result<(), rusqlite::Error> {
        
	// prepare the query to delete the payload with payload_id = id
	let query = "DELETE FROM payloads WHERE payload_id = ?";

        let affected_rows = conn.execute(query, [id])?;

        if affected_rows == 0 {
            println!("No payload found with ID: {}", id);
        } else {
            println!("Payload with ID: {} deleted successfully", id);
        }

        Ok(())
    }
}


