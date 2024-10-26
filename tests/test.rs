use std::fs;
use rusqlite::{Connection, Result as SqlResult, Error as SqlError};
use cindy_gao_mini_week8::{extract, load, read_query, update_query, delete_query, sorting_change};

/// Helper function to set up a clean database for each test
fn setup_database() -> Result<(), Box<dyn std::error::Error>> {
    let dataset_path = "data/murder_2015_final.csv";

    // Ensure dataset is present
    if !fs::metadata(dataset_path).is_ok() {
        let url = "https://raw.githubusercontent.com/fivethirtyeight/data/master/murder_2016/murder_2015_final.csv";
        extract(url, dataset_path)?;
    }

    // Connect to SQLite database
    let conn = Connection::open("Murder2015.db")?;

    // Ensure a fresh start by dropping the table if it exists
    conn.execute("DROP TABLE IF EXISTS Murder2015", [])
        .map_err(|e| Box::<dyn std::error::Error>::from(e))?;
    
    // Load data from the CSV file
    load(dataset_path).map_err(|e| Box::<dyn std::error::Error>::from(e))?;

    // Insert "Chicago" for testing, ignoring if it already exists
    conn.execute(
        "INSERT OR IGNORE INTO Murder2015 (city, state, murders_2014, murders_2015, change) VALUES (?, ?, ?, ?, ?)",
        ("Chicago", "Illinois", 411, 478, 67),
    ).map_err(|e| Box::<dyn std::error::Error>::from(e))?;

    // Ensure the database connection is closed
    conn.close().map_err(|(_, e)| Box::<dyn std::error::Error>::from(e))?;

    Ok(())
}

#[test]
fn test_extract() {
    let url = "https://raw.githubusercontent.com/fivethirtyeight/data/master/murder_2016/murder_2015_final.csv";
    let file_path = "data/murder_2015_final.csv";
    
    // Test extraction
    let result = extract(url, file_path);
    assert!(result.is_ok(), "Failed to extract data");
    
    // Verify the file was downloaded
    assert!(fs::metadata(file_path).is_ok(), "File was not found after extraction");
}

#[test]
fn test_load() {
    // Ensure the file exists in the expected directory
    let file_path = "data/murder_2015_final.csv";
    assert!(fs::metadata(file_path).is_ok(), "Dataset file not found in 'data' directory");

    // Test load function
    let result = load(file_path);
    assert!(result.is_ok(), "Failed to load data into the database");

    // Confirm the database file is created
    assert!(fs::metadata("Murder2015.db").is_ok(), "Database file was not created");
}

#[test]
fn test_read_query() {
    setup_database().expect("Failed to set up the database");
    let result = read_query();
    assert!(result.is_ok(), "read_query function did not execute as expected");
}

#[test]
fn test_update_query() {
    setup_database().expect("Failed to set up the database");
    let result = update_query();
    assert!(result.is_ok(), "Failed to update the 'change' field for 'Chicago'");

    // Verify that Chicago's 'change' field was updated
    let conn = Connection::open("Murder2015.db").expect("Failed to open database");
    let mut stmt = conn.prepare("SELECT change FROM Murder2015 WHERE city = 'Chicago'").expect("Failed to prepare statement");
    let change: i32 = stmt.query_row([], |row| row.get(0)).expect("Failed to fetch row");
    assert_eq!(change, 60, "Chicago's 'change' field was not updated correctly");
}

#[test]
fn test_delete_query() {
    setup_database().expect("Failed to set up the database");
    let result = delete_query();
    assert!(result.is_ok(), "Failed to delete record for 'Chicago' from database");

    // Verify the deletion
    let conn = Connection::open("Murder2015.db").expect("Failed to open database");
    let mut stmt = conn.prepare("SELECT COUNT(*) FROM Murder2015 WHERE city = 'Chicago'").expect("Failed to prepare statement");
    let count: i32 = stmt.query_row([], |row| row.get(0)).expect("Failed to fetch row");
    assert_eq!(count, 0, "Record for 'Chicago' was not deleted");
}

#[test]
fn test_sorting_change() {
    setup_database().expect("Failed to set up the database");
    let result = sorting_change();
    assert!(result.is_ok(), "sorting_change function did not execute as expected");
}

#[test]
fn cleanup_test_data() {
    if let Err(e) = fs::remove_file("Murder2015.db") {
        eprintln!("Error cleaning up database file: {:?}", e);
    }
}
