use futures_util::StreamExt;
use uuid::Uuid;
use async_nats::Client;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Location {
    lat: f64,
    lon: f64,
}

#[derive(Serialize, Deserialize)]
struct SearchQuery {
    id: String,
    loc: Location,
    rad: u32,
}

#[tokio::main]
async fn main() -> Result<(), async_nats::Error> {
    let client = async_nats::connect("localhost").await?;
    
        let id = Uuid::new_v4();
        send_search_query(&client, id).await?;

    let mut  subscriber = client.subscribe("spots".to_string()).await?;
    
    while let Some(msg) = subscriber.next().await {
        println!("{msg:?}");
    }

    Ok(())
}

async fn send_search_query(client: &Client, id: Uuid) -> Result<(), async_nats::Error> {
    let query = SearchQuery {
        id: id.to_string(),
        loc: Location { /* home 48.81478,9.58191 */ /* aussicht 48.81862, 9.5873 */
            lat: 48.81862,
            lon:  9.5873
        },
        rad: 500,
    };

    let msg = serde_json::to_string(&query)?;
    client.publish(
        "search".to_string(),
        msg.into(),
    ).await?;
    
    Ok(())
}
