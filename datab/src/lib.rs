use rusqlite::{Connection,
               params,
               Result};

// inner part
struct Server{
    id: u64,
    name: String
}

fn read_db() -> Result<Connection>{
    let db = Connection::open("content/global.db")
        .expect("Create a global.db sqlite3 file in content directory.");
    Ok(db)
}

// pub part

// pub fn check_servers_table(){
//     let db = read_db();
//     db.execute("CREATE TABLE IF NOT EXISTS servers (guild_id INTEGER, guild_name)")
// }

//redo the whole module from sample