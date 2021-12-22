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
    // this block creates interactions in discord chat menu
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        let new = vec![
            ("ping", "blah blah blah"),
            ("ring", "ding-dong!"),
            ("duck", "quack")
        ]; //redo this with commands from database


        if let Interaction::ApplicationCommand(command) = interaction {

            //checks the commands list from vector of tuples
            let mut content = command.data.name.as_str();
            for item in 0..new.len(){
                if content == new[item].0.to_string(){
                    content = new[item].1;

                    let &from_guild = command.guild_id.unwrap().as_u64(); // gets the guild_id of msg recieved
                    println!("{}", from_guild); // prints guild id from where command was called
                };
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
        let guild_id = GuildId(682536422116294656);

        // let guilds_num = ctx.cache.guilds().await.len();
        // println!("Number of guilds in the Cache: {}", guilds_num);
        //
        //
        // let guilds = &ctx.cache.guilds().await;
        // for guild in guilds{
        //     // let tname = "test";
        //     println!("{}", &guild);
        //     datab::check_servers_tables(&connection, &guild.to_string());
        // }
        // println!("Guilds in the Cache: {:?}", guilds);

        // let guilds2 = ctx.cache.guild(682536422116294656).await.unwrap();
        // println!("Guild info: {:?}", guilds2.name);



        let commands = GuildId::set_application_commands(&guild_id, &ctx.http, |commands| {
            commands
                .create_application_command(|command| {
                    command.name("ping").description("A ping command")
                })
                .create_application_command(|command| {
                    command.name("ring").description("Ring a bell")
                })
                .create_application_command(|command|{
                    command.name("duck").description("duck")
                })
        }).await;
        println!("I now have the following guild slash commands: {:#?}", commands);

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
