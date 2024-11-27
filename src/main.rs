use chrono::prelude::*;
use chrono::DateTime;

use dotenv::dotenv;

use polars::prelude::*;
use polars::prelude::JsonReader;
use polars::prelude::ParquetWriter;
use std::env;
use std::io::Cursor;

use serde::Serialize;

use atrium_api::types::string::{AtIdentifier, Datetime};
use bsky_sdk::BskyAgent;

#[derive(Serialize)]
struct Follower {
    did: String,
    handle: String,
    description: Option<String>,
    created_at: Option<Datetime>,
    index_at: Option<Datetime>,
    avatar: Option<String>
}

struct LabelData {
    cid: String,
    cts: DateTime<Utc>,
    exp: String,
    neg: String,
    sig: String,
    src: String,
    uri: String,
    val: String,
    var: String
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let env_bluesky_username = env::var("BLUESKY_USERNAME");
    let env_bluesky_app_password = env::var("BLUESKY_APP_PASSWORD");

    let mut bluesky_username = String::new();
    let mut bluesky_app_password = String::new();

    let mut vec_followers = Vec::new();

    match env_bluesky_username {
        Ok(val) => bluesky_username=val,
        Err(e) => println!("Error username: {}", e),
    }

    match env_bluesky_app_password {
        Ok(val) => bluesky_app_password=val,
        Err(e) => println!("Error app password: {}", e)
    }
    
    let agent = BskyAgent::builder().build().await?;
    let session = agent.login(bluesky_username, bluesky_app_password).await?;

    let followers = agent
        .api
        .app
        .bsky
        .graph
        .get_followers(
            atrium_api::app::bsky::graph::get_followers::ParametersData {
                actor: AtIdentifier::Did(session.data.did),
                cursor: None, 
                limit: None
            }
            .into(),
        ).await?;

    for follower in followers.data.followers.iter() {
        
        let new_follower = Follower {
            did: String::from(follower.did.to_string()),
            handle: String::from(follower.handle.to_string()),
            description: follower.description.clone(),
            created_at: follower.created_at.clone(),
            index_at: follower.indexed_at.clone(),
            avatar: follower.avatar.clone()
        }; 

        println!("{:?}", new_follower.did);
        vec_followers.push(new_follower);

        for labels in follower.labels.iter() {
            for label in labels.iter() {
                println!("{:?}", label.data)
            }
        }
    }

    let json = serde_json::to_string(&vec_followers).unwrap();

    let cursor = Cursor::new(json);

    let mut df = JsonReader::new(cursor).finish().unwrap();

    let mut file = std::fs::File::create("test.parquet").unwrap();

    ParquetWriter::new(&mut file).finish(&mut df).unwrap();
    
//    println!("{:?}", followers);
    Ok(())
}
