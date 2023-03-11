use config::CONFIG;
use serenity::async_trait;
use serenity::model::prelude::{Reaction, ReactionType};
use serenity::prelude::*;
use serenity::framework::standard::{StandardFramework, CommandResult};


pub mod private_channel;
pub mod config;

struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn reaction_add(&self, ctx: Context, add_reaction: Reaction) {
        if let ReactionType::Unicode(code) = add_reaction.clone().emoji {
            match code.as_str() {
                "💬" => {
                    if let Err(err) = private_channel::create_private_channel(&ctx, &add_reaction).await {
                        println!("Private channel created error: {:?}", err);
                    } else {
                        println!("Private channel created");
                    }
                     
                },
                _ => (),
            }    
        }
    }
}

#[tokio::main]
async fn main() {
    let framework = StandardFramework::new();
        /*.configure(|c| c.prefix("!")) // set the bot's prefix to "!"
        .group(&GENERAL_GROUP); */

    
    // Login with a bot token from the environment
    let token = CONFIG.token();
    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT | GatewayIntents::GUILD_MESSAGE_REACTIONS;
    let mut client = Client::builder(token, intents)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    // start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("An error occurred while running the client. Be sure you have specified a correct token: {why:?}");
    }
}

