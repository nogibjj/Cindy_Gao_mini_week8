use csv::ReaderBuilder;
use reqwest::blocking::get;
use rusqlite::{params, Connection, Result};
use std::error::Error;
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;

/// Extract a dataset from a URL to a specified file path.
pub fn extract(url: &str, file_path: &str) -> Result<(), Box<dyn Error>> {
    if let Some(parent) = Path::new(file_path).parent() {
        fs::create_dir_all(parent)?;
    }
    let response = get(url)?;
    if response.status().is_success() {
        let mut file = File::create(file_path)?;
        file.write_all(&response.bytes()?)?;
        println!("File successfully downloaded to {}", file_path);
    } else {
        println!(
            "Failed to retrieve the file. HTTP Status Code: {:?}",
            response.status()
        );
    }
    Ok(())
}

/// Transforms and loads data from a CSV file into the local SQLite3 database
pub fn load(dataset: &str) -> Result<(), Box<dyn Error>> {
    println!("Current directory: {:?}", std::env::current_dir()?);
    let file = File::open(dataset)?;
    let mut rdr = ReaderBuilder::new().has_headers(true).from_reader(file);
    let conn = Connection::open("Murder2015.db")?;

    conn.execute("DROP TABLE IF EXISTS Murder2015", [])
        .map_err(|e| Box::new(e) as Box<dyn Error>)?;
    conn.execute(
        "CREATE TABLE Murder2015 (city TEXT, state TEXT, murders_2014 INTEGER, murders_2015 INTEGER, change INTEGER)",
        [],
    ).map_err(|e| Box::new(e) as Box<dyn Error>)?;

    let mut stmt = conn.prepare("INSERT INTO Murder2015 (city, state, murders_2014, murders_2015, change) VALUES (?, ?, ?, ?, ?)")?;
    for result in rdr.records() {
        let record = result?;
        stmt.execute(params![
            record[0].to_string(),
            record[1].to_string(),
            record[2].parse::<i32>()?,
            record[3].parse::<i32>()?,
            record[4].parse::<i32>()?
        ])
        .map_err(|e| Box::new(e) as Box<dyn Error>)?;
    }
    println!("Data successfully loaded into Murder2015.db");
    Ok(())
}

/// Query the database for the top 5 rows of the Murder2015 table
pub fn read_query() -> Result<(), Box<dyn Error>> {
    let conn = Connection::open("Murder2015.db")?;
    let mut stmt = conn
        .prepare("SELECT * FROM Murder2015 LIMIT 5")
        .map_err(|e| Box::new(e) as Box<dyn Error>)?;
    let rows = stmt
        .query_map([], |row| {
            Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))
        })
        .map_err(|e| Box::new(e) as Box<dyn Error>)?;

    println!("Top 5 rows of the Murder2015 table:");
    for row in rows {
        println!("{:?}", row?);
    }
    Ok(())
}

/// Update the 'change' value for 'Chicago' in the Murder2015 table
pub fn update_query() -> Result<(), Box<dyn Error>> {
    let conn = Connection::open("Murder2015.db")?;
    conn.execute(
        "UPDATE Murder2015 SET change = 60 WHERE city = 'Chicago'",
        [],
    )
    .map_err(|e| Box::new(e) as Box<dyn Error>)?;
    println!("Update Success");
    Ok(())
}

/// Delete the row where city is 'Chicago' in the Murder2015 table
pub fn delete_query() -> Result<(), Box<dyn Error>> {
    let conn = Connection::open("Murder2015.db")?;
    conn.execute("DELETE FROM Murder2015 WHERE city = 'Chicago'", [])
        .map_err(|e| Box::new(e) as Box<dyn Error>)?;
    println!("Delete Success");
    Ok(())
}

/// Sort the Murder2015 table by 'change' and display 'state' and 'city' columns
pub fn sorting_change() -> Result<(), Box<dyn Error>> {
    let conn = Connection::open("Murder2015.db")?;
    let mut stmt = conn
        .prepare("SELECT state, city FROM Murder2015 ORDER BY change")
        .map_err(|e| Box::new(e) as Box<dyn Error>)?;
    let rows = stmt
        .query_map([], |row| {
            Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))
        })
        .map_err(|e| Box::new(e) as Box<dyn Error>)?;

    println!("Sorted by 'change':");
    for row in rows {
        println!("{:?}", row?);
    }
    Ok(())
}
