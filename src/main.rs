mod commands;

use std::env;

use serenity::{
    async_trait,
    framework::{
        StandardFramework,
        standard::macros::group,
    },
    model::{gateway::*, channel::GuildChannel},
    prelude::*,
};

use commands::{
    general::*,
    moderation::*,
};

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} si è avviato!", ready.user.name);
    }

    async fn channel_create(&self, _ctx: Context, channel: &GuildChannel) {
        println!("{} è stata creata", channel.name);
    }
}

#[group]
#[commands(ping, kick, ban, unban)]
struct General;

#[tokio::main]
async fn main() {

    dotenv::dotenv().expect("Impossibile caricare il file .env");

    let token = env::var("DISCORD_TOKEN").expect("Nessun token dato");

    let framework = StandardFramework::new()
        .configure(|c| c
            .prefix("drk!"))
        .group(&GENERAL_GROUP);

    let mut client = Client::builder(&token).framework(framework).event_handler(Handler).await.expect("Errore durante la creazione del client");

    if let Err(why) = client.start().await {
        println!("Errore: {:?}", why);
    }
}
