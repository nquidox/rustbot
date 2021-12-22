// project mods
use datab;
use config_reader;

// crates
use serenity::{
    async_trait,
    model::{
        gateway::Ready,
        channel::Message,
        id::GuildId,
        guild::Guild,
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
use serenity::model::prelude::Connection;
// end of import


struct Handler;


#[async_trait]
impl EventHandler for Handler {
    // custom prefix commands

    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "!ping" {
            let cont = datab::get_command("duck").unwrap();

            if let Err(why) = msg.channel_id.say(&ctx.http, cont).await {
                println!("Error sending message: {:?}", why);
            }
        }
    }

    // slash commands
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            //checks the commands list from vector of tuples
            let commands_list = &datab::commands_list().unwrap();
            let mut content = command.data.name.as_str();
            for i in 0..commands_list.len(){
                if content == commands_list[i].2{
                    content = &*commands_list[i].4
                }
            }

            if let Err(why) = command
                .create_interaction_response(&ctx.http, |response| {
                    response
                        .kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|message| message.content(content))
                })
                .await
            {
                println!("Cannot respond to slash command: {}", why);
            }
        }
    }


    // ready function
    async fn ready(&self, ctx: Context, ready: Ready) {
        // db init
        datab::check_existence();

        println!("{} is connected!", ready.user.name);

        let commands_list = &datab::commands_list().unwrap();
        for i in 0..commands_list.len() {
            println!("Iteration {}, commands list len {}", i, commands_list.len());
            let guild_id = GuildId(commands_list[i].0.clone());

            // creates only 1 command because of rewrite on every iteration, needs fixing
            let com = GuildId::set_application_commands(&guild_id, &ctx.http, |commands|{
                commands.create_application_command(|command|{
                    command.name(&commands_list[i].2).description(&commands_list[i].3)
                })
            }).await;
            println!("{:#?}", com);
        }
    }
}


#[tokio::main]
async fn main() {

    let token = &*config_reader::read_global_config()[0];
    let app_id = config_reader::read_global_config()[1].parse::<u64>().unwrap();

    let mut client = Client::builder(token)
        .event_handler(Handler)
        .application_id(app_id)
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
