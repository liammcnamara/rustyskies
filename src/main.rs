use dotenv::dotenv;
use std::env;

use atrium_api::types::string::AtIdentifier;
use bsky_sdk::BskyAgent;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let env_bluesky_username = env::var("BLUESKY_USERNAME");
    let env_bluesky_app_password = env::var("BLUESKY_APP_PASSWORD");

    let mut bluesky_username = String::new();
    let mut bluesky_app_password = String::new();

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

    println!("{:?}", followers);
    Ok(())
}
