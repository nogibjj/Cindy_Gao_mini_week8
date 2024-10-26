use rusqlite::Connection;
use std::error::Error;
use std::fs;
use cindy_gao_mini_week8::{extract, load, read_query, update_query, delete_query, sorting_change};

fn setup_database() -> Result<(), Box<dyn Error>> {
    let dataset_path = "data/murder_2015_final.csv";
    
    // Ensure the file exists in the expected directory
    if !fs::metadata(dataset_path).is_ok() {
        return Err("Dataset file not found in 'data' directory.".into());
    }
    
    // Connect to the database and reset it
    let conn = Connection::open("Murder2015.db")?;
    conn.execute("DROP TABLE IF EXISTS Murder2015", [])?;
    load(dataset_path)?;
    Ok(())
}

#[test]
fn test_extract() {
    let url = "https://raw.githubusercontent.com/fivethirtyeight/data/master/murder_2016/murder_2015_final.csv";
    let file_path = "data/murder_2015_final.csv";
    
    // Test extraction
    assert!(extract(url, file_path).is_ok(), "Failed to extract data");
    
    // Verify the file was downloaded
    assert!(fs::metadata(file_path).is_ok(), "File was not found after extraction");
}

#[test]
fn test_load() {
    let file_path = "data/murder_2015_final.csv";
    
    // Ensure the file exists in the expected directory
    assert!(fs::metadata(file_path).is_ok(), "Dataset file not found in 'data' directory");

    // Test load
    assert!(load(file_path).is_ok(), "Failed to load data into the database");

    // Confirm the database file is created
    assert!(fs::metadata("Murder2015.db").is_ok(), "Database file was not created");
}

#[test]
fn test_read_query() {
    setup_database().expect("Failed to set up the database");
    assert!(read_query().is_ok(), "Failed to read query from database");
}

#[test]
fn test_update_query() {
    setup_database().expect("Failed to set up the database");
    assert!(update_query().is_ok(), "Failed to update database record");

    // Verify the update
    let conn = Connection::open("Murder2015.db").expect("Failed to open database");
    let mut stmt = conn.prepare("SELECT change FROM Murder2015 WHERE city = 'Chicago'").expect("Failed to prepare statement");
    let change: i32 = stmt.query_row([], |row| row.get(0)).expect("Failed to fetch row");
    assert_eq!(change, 60, "Expected 'change' to be 60 for 'Chicago'");
}

#[test]
fn test_delete_query() {
    setup_database().expect("Failed to set up the database");
    assert!(delete_query().is_ok(), "Failed to delete record from database");

    // Verify the deletion
    let conn = Connection::open("Murder2015.db").expect("Failed to open database");
    let mut stmt = conn.prepare("SELECT COUNT(*) FROM Murder2015 WHERE city = 'Chicago'").expect("Failed to prepare statement");
    let count: i32 = stmt.query_row([], |row| row.get(0)).expect("Failed to fetch row");
    assert_eq!(count, 0, "Record for 'Chicago' was not deleted");
}

#[test]
fn test_sorting_change() {
    setup_database().expect("Failed to set up the database");
    assert!(sorting_change().is_ok(), "Failed to sort and retrieve data from database");
}

#[test]
fn cleanup_test_data() {
    let _ = fs::remove_file("Murder2015.db");
}
