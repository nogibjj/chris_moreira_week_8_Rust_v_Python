use rusqlite::{params, Connection, Result};
use csv::ReaderBuilder;
use std::fs::File;
use std::error::Error;
use std::io::Write;
use reqwest;
use std::time::Instant;

pub fn extract(url: &str, file_path: &str) -> Result<(), Box<dyn Error>> {
    let response = reqwest::blocking::get(url)?.bytes()?;
    let mut file = File::create(file_path)?;
    file.write_all(&response)?;
    println!("File downloaded successfully to {}", file_path);
    Ok(())
}

pub fn create_table(conn: &Connection) -> Result<()> {
    let start_time = Instant::now();
    conn.execute(
        "CREATE TABLE IF NOT EXISTS SpotifyDB (
            id INTEGER PRIMARY KEY,
            track_name TEXT,
            artist_name TEXT,
            artist_count INTEGER,
            released_year INTEGER,
            released_month INTEGER,
            released_day INTEGER,
            in_spotify_playlists INTEGER,
            in_spotify_charts INTEGER,
            streams INTEGER,
            in_apple_playlists INTEGER,
            key TEXT,
            mode TEXT,
            danceability_percent INTEGER,
            valence_percent INTEGER,
            energy_percent INTEGER,
            acousticness_percent INTEGER,
            instrumentalness_percent INTEGER,
            liveness_percent INTEGER,
            speechiness_percent INTEGER,
            cover_url TEXT
        )",
        [],
    )?;
    let duration = start_time.elapsed();
    println!("Table created successfully in {:?}", duration);
    Ok(())
}

pub fn load_data_from_csv(
    conn: &Connection,
    file_path: &str,
) -> Result<(), Box<dyn Error>> {
    let file = File::open(file_path)?;
    let mut rdr = ReaderBuilder::new().from_reader(file);
    let insert_query = "INSERT INTO SpotifyDB (
        track_name, artist_name, artist_count, released_year,
        released_month, released_day, in_spotify_playlists,
        in_spotify_charts, streams, in_apple_playlists, key, mode,
        danceability_percent, valence_percent, energy_percent,
        acousticness_percent, instrumentalness_percent, 
        liveness_percent, speechiness_percent, cover_url
    ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)";
    for result in rdr.records() {
        let record = result?;
        conn.execute(
            insert_query,
            params![
                &record[0], &record[1], &record[2], &record[3],
                &record[4], &record[5], &record[6], &record[7],
                &record[8], &record[9], &record[10], &record[11],
                &record[12], &record[13], &record[14], &record[15],
                &record[16], &record[17], &record[18], &record[19]
            ],
        )?;
    }
    println!("Data loaded successfully from '{}'", file_path);
    Ok(())
}

pub fn query_create(conn: &Connection) -> Result<()> {
    let sql = "INSERT INTO SpotifyDB (
        track_name, artist_name, artist_count, released_year,
        released_month, released_day, in_spotify_playlists,
        in_spotify_charts, streams, in_apple_playlists, key, mode,
        danceability_percent, valence_percent, energy_percent,
        acousticness_percent, instrumentalness_percent, 
        liveness_percent, speechiness_percent, cover_url
    ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)";

    conn.execute(
        sql,
        params![
            "Sample Song", "Sample Artist", 1, 2024, 10, 3, 400, 100,
            150000000, 200, "C#", "Major", 60, 70, 80, 10, 0, 20, 5,
            "https://sampleurl.com"
        ],
    )?;
    println!("Record inserted successfully.");
    Ok(())
}

pub fn query_read(conn: &Connection) -> Result<()> {
    let mut stmt = conn.prepare("SELECT * FROM SpotifyDB LIMIT 10")?;
    let rows = stmt.query_map([], |row| {
        Ok((row.get::<_, i32>(0)?, row.get::<_, String>(1)?))
    })?;
    for row in rows {
        println!("{:?}", row?);
    }
    Ok(())
}

pub fn query_update(conn: &Connection) -> Result<()> {
    conn.execute(
        "UPDATE SpotifyDB SET artist_name = 'Chris' WHERE id = 3",
        [],
    )?;
    println!("Record updated successfully.");
    Ok(())
}

pub fn query_delete(conn: &Connection) -> Result<()> {
    conn.execute("DELETE FROM SpotifyDB WHERE id = 5", [])?;
    println!("Record deleted successfully.");
    Ok(())
}
