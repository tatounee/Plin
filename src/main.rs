mod clan;
mod cmd;
mod data;
mod handler;
mod post;
mod river_race;

use dotenv::dotenv;
use reqwest::{
    header::{HeaderMap, ACCEPT, AUTHORIZATION},
    Client as ReqwClient,
};
use serenity::{
    model::{channel::Message, id::ChannelId},
    Client as DiscordClient,
};

use std::env;

use handler::Handler;

ctx_data!(
    UpdateDuration => u64,
    IsNewMessage => bool,
    UpdatePost => bool,
    Run => bool,
    Day => String,
    PostChannelId => ChannelId,
    Post => Message,
    ClanTag => String,
    PeriodIndex => i32,
);

const TIME_FRAGMENTATION: u64 = 5;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let plin_token =
        env::var("PLIN_DEV_DISCORD_TOKEN").expect("Expected a Discord token in the environment");
    let cr_token =
        env::var("PLIN_CR_TOKEN").expect("Expected a Clash Royale token in the environment");

    let mut header = HeaderMap::new();
    header.insert(ACCEPT, "application/json".parse().unwrap());
    header.insert(
        AUTHORIZATION,
        format!("Bearer {}", cr_token).parse().unwrap(),
    );
    let client = ReqwClient::new();

    let handler = Handler::new(client, header);

    let mut bot = DiscordClient::builder(&plin_token)
        .event_handler(handler)
        .await
        .expect("Err creating client");

    if let Err(why) = bot.start().await {
        println!("Client error: {:?}", why);
    }
}
