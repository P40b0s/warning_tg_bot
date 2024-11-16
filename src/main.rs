mod users;
mod settings;
mod app_state;
mod api_key;
use std::sync::Arc;

use app_state::AppState;
use teloxide::{dispatching::dialogue::GetChatId, payloads::GetChatMemberCount, prelude::*, repls::CommandReplExt, utils::command::BotCommands};
use users::{State, Status, UsersState};
use utilites::Date;
extern crate utilites;

#[tokio::main]
async fn main() 
{
    logger::StructLogger::new_default();
    logger::info!("Starting command bot...");

    let bot = Bot::new(api_key::KEY);
    let app_state = AppState::new();
    Dispatcher::builder(
        bot,
        dptree::entry()
            .branch(
                Update::filter_message()
                    .filter_command::<Command>()
                    .endpoint(cmd_handler),
            )
            .branch(Update::filter_message().endpoint(text_handler)),
    )
    .dependencies(dptree::deps![app_state])
    .enable_ctrlc_handler()
    .build()
    .dispatch()
    .await;
}

#[derive(BotCommands, Clone, Debug)]
#[command(rename_rule = "lowercase", description = "Поддерживаются команды:")]
enum Command {
    #[command(description = "Показать это сообщение.")]
    Help,
    #[command(description = "Установить количество человек которое необходимо оплюсить.")]
    SetCount(u8),
    #[command(description = "Сколько человек необходимо оплюсить?")]
    GetCount,
    #[command(description = "handle a username and an age.", parse_with = "split")]
    UsernameAndAge { username: String, age: u8 },
}
async fn text_handler(bot: Bot, msg: Message, state: Arc<AppState>) -> ResponseResult<()> 
{
    logger::debug!("пришел текст {:?}",  msg.text());
    
    match msg.text()
    {
        Some(text) => 
        {
            match text
            {
                "+" => 
                {
                    if let Some(user) = msg.from.as_ref()
                    {
                        let date = msg.date.clone().naive_local();
                        let d = Date::from(date);
                        
                        let user = users::User::new(user.id.0, user.first_name.clone(), user.username.clone(), d);
                        let st = State::new(Some(user));
                        let mut guard = state.users_state.write().await;
                        guard.add_of_replace_status(st);
                        logger::debug!("state:{:?}", guard);
                        bot.send_message(msg.chat.id, guard.to_string())
                        .await?;
                    };
                    ()
                },
                "-" => 
                {
                    if let Some(user) = msg.from.as_ref()
                    {
                        let date = msg.date.clone().naive_local();
                        let d = Date::from(date);
                        let  user = users::User::new(user.id.0, user.first_name.clone(), user.username.clone(), d);
                        let mut st = State::new(Some(user));
                        st.change_status(Status::Minus);
                        let mut guard = state.users_state.write().await;
                        guard.add_of_replace_status(st);
                        logger::debug!("state:{:?}", guard);
                        bot.send_message(msg.chat.id, guard.to_string())
                        .await?;
                    };
                    ()
                },
                _ => ()
            }
        }
        None => {
            bot.send_message(msg.chat.id, "Send me plain text.").await?;
        }
    }
    Ok(())
}

async fn cmd_handler(bot: Bot, msg: Message, cmd: Command, state: Arc<AppState>) -> ResponseResult<()> 
{
    logger::debug!("Пришла команда {:?}",  msg.text());
    match cmd 
    {
        Command::Help => bot.send_message(msg.chat.id, Command::descriptions().to_string()).await?,
        // Command::Username(username) => 
        // {
        //     bot.send_message(msg.chat.id, format!("Your username is @{username}.")).await?
        // }
        Command::GetCount => 
        {
            bot.send_message(msg.chat.id, format!("Количество отслеживаемых человек: {}", state.users_state.read().await.get_count()))
        .await?
        }
        Command::SetCount(cnt) => 
        {
            let mut guard = state.users_state.write().await;
            guard.set_count(cnt);
            drop(guard);
            let mut settings_guard = state.settings.write().await;
            settings_guard.count = cnt;
            settings_guard.save();
            drop(settings_guard);
            bot.send_message(msg.chat.id, format!("Количество отслеживаемых человек установлено на: {}", cnt))
        .await?
        }
        _ =>  bot.send_message(msg.chat.id, format!("Your username is and age is. {}", state.users_state.read().await.get_count()))
        .await?
    };

    Ok(())
}