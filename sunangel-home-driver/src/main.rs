use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Location {
    lat: i32,
    lon: i32,
}

#[derive(Serialize, Deserialize)]
struct SearchQuery {
    loc: Location,
    rad: u32,
}

#[tokio::main]
async fn main() -> Result<(), async_nats::Error> {
    let connection = async_nats::connect("localhost").await?;
    
    let query = SearchQuery { loc: Location { lat: 48, lon: 9 }, rad: 3_000 };
    let msg = serde_json::to_string(&query)?;
    connection.publish("search".to_string(), msg.into()).await?;

    Ok(())
}
