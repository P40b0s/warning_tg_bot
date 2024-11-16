use teloxide::{
    prelude::*,
    types::Update,
    utils::command::BotCommands,
};

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("Starting test bot...");

    let bot = Bot::from_env().auto_send();

    let handler = dptree::entry()
        .branch(Update::filter_my_chat_member().endpoint(a))
        .branch(Update::filter_message().filter_command::<Cmd>().endpoint(b))
        .branch(Update::filter_message().endpoint(c));

    Dispatcher::builder(bot, handler).build().setup_ctrlc_handler().dispatch().await;
}

#[derive(BotCommands, Clone)]
#[command(rename = "lowercase")]
enum Cmd { B }

async fn a() -> Result<(), ()> {
    println!("a");
    Ok(())
}
async fn b() -> Result<(), ()> {
    println!("b");
    Ok(())
}
async fn c() -> Result<(), ()> {
    println!("c");
    Ok(())
}