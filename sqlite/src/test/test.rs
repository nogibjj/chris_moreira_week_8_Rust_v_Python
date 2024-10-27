use sqlite::{extract, create_table, load_data_from_csv, query_create, query_read};
use rusqlite::Connection;

#[test]
fn test_extract() {
    let url = "https://raw.githubusercontent.com/nogibjj/chris_moreira_week5_python_sql_db_project/main/data/Spotify_Most_Streamed_Songs.csv";
    let file_path = "../data/Spotify_Most_Streamed_Songs.csv";
    
    let result = extract(url, file_path);
    assert!(result.is_ok(), "Extract function failed");

    assert!(std::fs::metadata(file_path).is_ok(), "File not found after extraction");
}

#[test]
fn test_create_table() {
    let conn = Connection::open_in_memory().expect("Failed to open in-memory database");
    
    let result = create_table(&conn);
    assert!(result.is_ok(), "Table creation failed");
}

#[test]
fn test_load_data_from_csv() {
    let conn = Connection::open_in_memory().expect("Failed to open in-memory database");
    
    // Ensure the table is created before loading data
    create_table(&conn).expect("Table creation failed");
    
    let file_path = "../data/Spotify_Most_Streamed_Songs.csv";
    let result = load_data_from_csv(&conn, file_path);
    assert!(result.is_ok(), "Loading data from CSV failed");
}

#[test]
fn test_query_create() {
    let conn = Connection::open_in_memory().expect("Failed to open in-memory database");
    
    // Create table and insert data
    create_table(&conn).expect("Table creation failed");
    let result = query_create(&conn);
    assert!(result.is_ok(), "Insert query failed");
}

#[test]
fn test_query_read() {
    let conn = Connection::open_in_memory().expect("Failed to open in-memory database");
    
    // Create table, load data, and test read operation
    create_table(&conn).expect("Table creation failed");
    query_create(&conn).expect("Data insertion failed");
    
    let result = query_read(&conn);
    assert!(result.is_ok(), "Read query failed");
}