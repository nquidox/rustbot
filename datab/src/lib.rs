use rusqlite::*;

const COMMANDS_TABLE: &str = "all_servers_commands";

struct Command{
    guild_id: u64,
    guild_name: String,
    command: String,
    description: String,
    content: String
}


fn connect() -> Result<Connection>{
    let db = Connection::open("content/global.db")
        .expect("Create a global.db sqlite3 file in content directory.");
    Ok(db)
}


pub fn check_existence(){
    let db = connect().expect("Unable to connect to database.");
    let sql = &*format!("CREATE TABLE IF NOT EXISTS '{}' (\
    guild_id INTEGER,\
    guild_name TEXT,\
    command TEXT,\
    description TEXT,\
    content TEXT)", COMMANDS_TABLE);
    db.execute(sql, []).expect("SQL query failed.");
}

// not actual, but left for future decisions
pub fn get_command(command: &str) -> Result<String> {
    let db = connect().expect("Unable to connect to database.");
    let sql = &*format!("SELECT content FROM '{}' WHERE command = :command", COMMANDS_TABLE);
    let mut stmt = db.prepare(sql)?;
    let mut rows = stmt.query(named_params! { ":command": command })?;
    let mut res = String::new();

    while let Some(row) = rows.next()? {
        res = row.get(0)?;
    }
    Ok(res)
}

pub fn commands_list() -> Result<Vec<(u64, String, String, String, String)>> {
    let mut com_list = Vec::new();

    let db = connect().expect("Unable to connect to database.");
    let sql = &*format!("SELECT * FROM '{}'", COMMANDS_TABLE);
    let mut stmt = db.prepare(sql)?;

    // looks unnecessary complicated but it works, refactor later
    let commands_iter = stmt.query_map([], |row| {
        Ok(Command {
            guild_id: row.get(0)?,
            guild_name: row.get(1)?,
            command: row.get(2)?,
            description: row.get(3)?,
            content: row.get(4)?
        })
    })?;

    for command in commands_iter {
        let res = (
            command.as_ref().unwrap().guild_id.clone(),
            command.as_ref().unwrap().guild_name.clone(),
            command.as_ref().unwrap().command.clone(),
            command.as_ref().unwrap().description.clone(),
            command.as_ref().unwrap().content.clone()
        );
        com_list.push(res);
    }

    Ok(com_list)
}