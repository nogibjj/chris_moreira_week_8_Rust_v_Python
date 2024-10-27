use sqlite::{
    extract, create_table, load_data_from_csv, query_create,
    query_read, query_update, query_delete,
};
use rusqlite::Connection;
use std::time::Instant;

fn main_results(
    conn: &Connection,
) -> Result<(), Box<dyn std::error::Error>> {
    // Ensure the data directory exists
    std::fs::create_dir_all("../data")?;

    let url = concat!(
        "https://raw.githubusercontent.com/nogibjj/",
        "chris_moreira_week5_python_sql_db_project/",
        "main/data/Spotify_Most_Streamed_Songs.csv"
    );
    let extract_result = extract(url, "../data/Spotify_Most_Streamed_Songs.csv");
    println!("Extract Result: {:?}", extract_result);

    let start_time = Instant::now();
    let create_result = create_table(&conn);
    let create_duration = start_time.elapsed();
    println!(
        "Create Table Result: {:?}, Duration: {:?}",
        create_result, create_duration
    );

    let load_result = load_data_from_csv(
        &conn,
        "../data/Spotify_Most_Streamed_Songs.csv",
    );
    println!("Load Result: {:?}", load_result);

    let create_query_result = query_create(&conn);
    println!("Query Create Result: {:?}", create_query_result);

    let read_result = query_read(&conn);
    println!("Read Result: {:?}", read_result);

    let update_result = query_update(&conn);
    println!("Update Result: {:?}", update_result);

    let delete_result = query_delete(&conn);
    println!("Delete Result: {:?}", delete_result);

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let conn = Connection::open("SpotifyDB.db")?;
    main_results(&conn)?;
    Ok(())
}
