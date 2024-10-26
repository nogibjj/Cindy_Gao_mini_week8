use rusqlite::{params, Connection, Result};
use std::error::Error;
use std::fs;

// Setup the database by dropping the existing table (if any) and creating a new one
fn setup_database() -> Result<(), Box<dyn Error>> {
    let conn = Connection::open("Murder2015.db")?;
    
    // Drop the table if it already exists to ensure a clean setup
    conn.execute("DROP TABLE IF EXISTS Murder2015", [])
        .map_err(|e| Box::new(e) as Box<dyn Error>)?;
    
    // Create the table
    conn.execute(
        "CREATE TABLE Murder2015 (city TEXT, state TEXT, murders_2014 INTEGER, murders_2015 INTEGER, change INTEGER)",
        [],
    ).map_err(|e| Box::new(e) as Box<dyn Error>)?;
    
    Ok(())
}

// Cleanup function to delete the database file after each test
fn cleanup_database() {
    if std::path::Path::new("Murder2015.db").exists() {
        std::fs::remove_file("Murder2015.db").expect("Failed to delete the database file");
    }
}

#[test]
fn test_update_query() {
    // Setup the database
    setup_database().expect("Failed to set up the database");

    // Insert a record for Chicago to test the update
    let conn = Connection::open("Murder2015.db").expect("Failed to open database");
    conn.execute(
        "INSERT INTO Murder2015 (city, state, murders_2014, murders_2015, change) VALUES (?1, ?2, ?3, ?4, ?5)",
        params!["Chicago", "Illinois", 411, 478, 67],
    ).expect("Failed to insert data");

    // Perform the update query
    conn.execute(
        "UPDATE Murder2015 SET change = 60 WHERE city = 'Chicago'",
        [],
    ).expect("Failed to update data");

    // Verify the update
    let mut stmt = conn.prepare("SELECT change FROM Murder2015 WHERE city = 'Chicago'").expect("Failed to prepare statement");
    let change: i32 = stmt.query_row([], |row| row.get(0)).expect("Failed to fetch row");
    assert_eq!(change, 60, "Chicago's 'change' value was not updated to 60");

    // Cleanup database file after test
    cleanup_database();
}

#[test]
fn test_delete_query() {
    // Setup the database
    setup_database().expect("Failed to set up the database");

    // Insert a record for Chicago to test the delete
    let conn = Connection::open("Murder2015.db").expect("Failed to open database");
    conn.execute(
        "INSERT INTO Murder2015 (city, state, murders_2014, murders_2015, change) VALUES (?1, ?2, ?3, ?4, ?5)",
        params!["Chicago", "Illinois", 411, 478, 67],
    ).expect("Failed to insert data");

    // Perform the delete query
    conn.execute("DELETE FROM Murder2015 WHERE city = 'Chicago'", [])
        .expect("Failed to delete data");

    // Verify the deletion
    let mut stmt = conn.prepare("SELECT COUNT(*) FROM Murder2015 WHERE city = 'Chicago'").expect("Failed to prepare statement");
    let count: i32 = stmt.query_row([], |row| row.get(0)).expect("Failed to fetch row");
    assert_eq!(count, 0, "Record for 'Chicago' was not deleted");

    // Cleanup database file after test
    cleanup_database();
}

#[test]
fn test_sorting_change() {
    // Setup the database
    setup_database().expect("Failed to set up the database");

    // Insert records to test sorting
    let conn = Connection::open("Murder2015.db").expect("Failed to open database");
    conn.execute("INSERT INTO Murder2015 (city, state, murders_2014, murders_2015, change) VALUES (?1, ?2, ?3, ?4, ?5)", params!["Chicago", "Illinois", 411, 478, 67]).expect("Failed to insert data");
    conn.execute("INSERT INTO Murder2015 (city, state, murders_2014, murders_2015, change) VALUES (?1, ?2, ?3, ?4, ?5)", params!["Houston", "Texas", 242, 303, 61]).expect("Failed to insert data");

    // Perform the sorting query
    let mut stmt = conn.prepare("SELECT state, city FROM Murder2015 ORDER BY change").expect("Failed to prepare statement");
    let rows = stmt.query_map([], |row| {
        Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))
    }).expect("Failed to map query");

    // Verify the sorting
    let sorted_data: Vec<(String, String)> = rows.map(|r| r.unwrap()).collect();
    assert_eq!(sorted_data[0].1, "Houston", "First city should be Houston based on change value");
    assert_eq!(sorted_data[1].1, "Chicago", "Second city should be Chicago based on change value");

    // Cleanup database file after test
    cleanup_database();
}

#[test]
fn test_read_query() {
    // Setup the database
    setup_database().expect("Failed to set up the database");

    // Insert sample records to test the read query
    let conn = Connection::open("Murder2015.db").expect("Failed to open database");
    conn.execute("INSERT INTO Murder2015 (city, state, murders_2014, murders_2015, change) VALUES (?1, ?2, ?3, ?4, ?5)", params!["Chicago", "Illinois", 411, 478, 67]).expect("Failed to insert data");
    conn.execute("INSERT INTO Murder2015 (city, state, murders_2014, murders_2015, change) VALUES (?1, ?2, ?3, ?4, ?5)", params!["Houston", "Texas", 242, 303, 61]).expect("Failed to insert data");

    // Perform the read query
    let mut stmt = conn.prepare("SELECT * FROM Murder2015 LIMIT 5").expect("Failed to prepare statement");
    let rows = stmt.query_map([], |row| {
        Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))
    }).expect("Failed to map query");

    // Verify the read operation
    let data: Vec<(String, String)> = rows.map(|r| r.unwrap()).collect();
    assert!(!data.is_empty(), "Data read from the database should not be empty");

    // Cleanup database file after test
    cleanup_database();
}
