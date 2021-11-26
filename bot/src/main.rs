use config_reader;
use serenity::{
    async_trait,
    model::{
        gateway::Ready,
        id::GuildId,
        interactions::{
            application_command::{
                ApplicationCommand,
                ApplicationCommandInteractionDataOptionValue,
                ApplicationCommandOptionType,
            },
            Interaction,
            InteractionResponseType,
        },
    },
    prelude::*,
};

struct Handler;

#[async_trait]
impl EventHandler for Handler {
        async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
async fn main() {

    let token = &*config_reader::read_config()[0];
    let app_id = config_reader::read_config()[1].parse::<u64>().unwrap();

    let mut client = Client::builder(token)
        .event_handler(Handler)
        .application_id(app_id)
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
