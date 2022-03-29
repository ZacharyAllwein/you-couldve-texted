use std::fs::{self, File};
use serde_json::{json, to_string_pretty};
use rusqlite::Connection;
use std::error::Error;

fn main() {
    setup().unwrap();
    
    let con = Connection::open("ytc_data.db").unwrap();

    con.execute(
        "insert into users (id, user_name) values (?1, ?2)",
        &[&"1", &"xanaphia"],
    ).unwrap();

    let stmt = con.prepare("select * from users").unwrap();

    let item = stmt.query_map([], |row| {
        Ok(row.get(0).unwrap())
    }).unwrap();


}

fn setup() -> Result<(), Box<dyn Error>>{

    let config_json = json!({
        "read":"false",
        "port":"7777",
        "database_path": "ytc_data.db"
    });
    
    fs::write("ytc_config.json", to_string_pretty(&config_json).unwrap())?;

    let con = Connection::open("ytc_data.db")?;

    con.execute(
        "create table users (
            id integer primary key,
            user_name text not null unique,
        )", [],
    )?;





    Ok(())
}