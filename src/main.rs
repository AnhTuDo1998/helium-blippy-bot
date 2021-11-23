use teloxide::{prelude::*, utils::command::BotCommand};
use std::error::Error;

// Define Telegram Bot commands here (with argument patterns)
#[derive(BotCommand)]
#[command(rename = "lowercase", description = "These commands are supported:")]
enum Command {
    #[command(description = "display this text.")]
    Help,
    #[command(description = "Get latest block with hotspot activity. Give hotspot name (in 3 letter, - seperated format) or address after the command")]
    GetLatestActivityBlock(String),
}

// Function to dispatch and handle different commands
async fn handle_command(
    cx: UpdateWithCx<AutoSend<Bot>, Message>,
    command: Command,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    match command {
        Command::Help => cx.answer(Command::descriptions()).await?,    
        //Command::GetLatestActivityBlock(String) => handle_latest_activity_block(String).await?,
        _ => cx.answer("Something wrong with your command? Who's a disobedient BOT ?").await?,
    };

    Ok(())
}

// Function handler for each command
// TODO: use regrex or another method to check if we getting address or hotspot name
async fn handle_latest_activity_block(hotspot_name: String) -> Result<(), Box<dyn Error + Send + Sync>> {
    // TODO: API call here
    Ok(())
}

#[tokio::main]
async fn main() {
    run().await;
}

async fn run() {
    teloxide::enable_logging!();
    log::info!("Starting helium_blippy_bot...");

    let bot = Bot::from_env().auto_send();

    let bot_name: String = String::from("Mr. Blippy");
    teloxide::commands_repl(bot, bot_name, handle_command).await;
}
