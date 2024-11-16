mod users;
mod settings;
mod app_state;
mod api_key;
mod timer;
mod keys;
use std::sync::Arc;

use app_state::AppState;
use teloxide::{dispatching::dialogue::GetChatId, prelude::*, types::{ParseMode, Recipient}, utils::command::BotCommands};
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
    //let handler = tokio::runtime::Runtime::new().unwrap();
    let sleep_state = Arc::clone(&app_state);
    //tokio::task::spawn_blocking(move || handler.spawn(async {timer::reset_pluses(sleep_state, 60).await}));
    tokio::spawn(async {timer::reset_pluses(sleep_state, 60*60*1).await});
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
    #[command(description = "*Показать это сообщение*")]
    Help,
    #[command(description = "*Установить количество человек которое необходимо оплюсить*")]
    SetCount(u8),
    #[command(description = "*Сколько человек необходимо оплюсить?*")]
    GetCount,
    #[command(description = "*Показать текущий статус*")]
    Status,
    #[command(description = "*Чтобы начать пользоваться ботом, необходимо выполнить эту команду с выданым ключом*")]
    Reg(String),
}
async fn text_handler(bot: Bot, msg: Message, state: Arc<AppState>) -> ResponseResult<()> 
{
    logger::debug!("пришел текст {:?}",  msg.text());
    let id = msg.chat_id();
    
    let chat_id = id.as_ref().map_or(0, |ident| ident.0);
    let bot = Arc::new(bot);
    match msg.text()
    {
        Some(text) => 
        {
            match text
            {
                "+" => 
                {
                    if is_reject(Arc::clone(&bot),msg.chat.id, Arc::clone(&state)).await
                    {
                        return Ok(());
                    }
                    if let Some(user) = msg.from.as_ref()
                    {
                        let date = msg.date.clone().naive_local();
                        let d = Date::from(date);
                        let user = users::User::new(user.id.0, user.first_name.clone(), user.username.clone(), d);
                        let st = State::new(Some(user));
                        let mut guard = state.users_states.write().await;
                        let user_state = guard.get_mut(&chat_id);
                        if let Some(us) = user_state
                        {
                            us.add_of_replace_status(st);
                        }
                        else 
                        {
                            let mut user_state = UsersState::default();
                            user_state.add_of_replace_status(st);
                            guard.insert(chat_id, user_state);
                        }
                        let result = guard.get(&chat_id).unwrap();
                        logger::debug!("state:{:?}", result);
                        bot.parse_mode(ParseMode::MarkdownV2).send_message(msg.chat.id, result.to_string())
                        .await?;
                        drop(guard);
                        state.save_users().await;
                        logger::debug!("{:?}", state.users_states.read().await)
                    };
                    ()
                },
                "-" => 
                {
                    if is_reject(Arc::clone(&bot),msg.chat.id, Arc::clone(&state)).await
                    {
                        return Ok(());
                    }
                    if let Some(user) = msg.from.as_ref()
                    {
                        let date = msg.date.clone().naive_local();
                        let d = Date::from(date);
                        let  user = users::User::new(user.id.0, user.first_name.clone(), user.username.clone(), d);
                        let mut st = State::new(Some(user));
                        st.change_status(Status::Minus);
                        let mut guard = state.users_states.write().await;
                        let user_state = guard.get_mut(&chat_id);
                        if let Some(us) = user_state
                        {
                            us.add_of_replace_status(st);
                        }
                        else 
                        {
                            let mut user_state = UsersState::default();
                            user_state.add_of_replace_status(st);
                            guard.insert(chat_id, user_state);
                        }
                        let result = guard.get(&chat_id).unwrap();
                        logger::debug!("state:{:?}", result);
                        bot.parse_mode(ParseMode::MarkdownV2).send_message(msg.chat.id, result.to_string())
                        .await?;
                        drop(guard);
                        state.save_users().await;
                        logger::debug!("{:?}", state.users_states.read().await)
                    };
                    ()
                },
                _ => ()
            }
        }
        None => 
        {
            //bot.send_message(msg.chat.id, "Ниче не понятно").await?;
        }
    }
    Ok(())
}

async fn cmd_handler(bot: Bot, msg: Message, cmd: Command, state: Arc<AppState>) -> ResponseResult<()> 
{
    logger::debug!("Пришла команда {:?}",  msg.text());
    let id = msg.chat_id();
    let chat_id = id.as_ref().map_or(0, |ident| ident.0);
    let bot = Arc::new(bot);
    match cmd 
    {
        Command::Help => 
        {
            let _sended =  bot.parse_mode(ParseMode::MarkdownV2).send_message(msg.chat.id, Command::descriptions().to_string()).await?;
            ()
        }
        Command::GetCount => 
        {
            if is_reject(Arc::clone(&bot),msg.chat.id, Arc::clone(&state)).await
            {
                return Ok(());
            }
            let guard = state.users_states.read().await;
            if let Some(u) = guard.get(&chat_id)
            {
                let count = u.get_count();
                let _sended = bot.parse_mode(ParseMode::MarkdownV2).send_message(msg.chat.id, format!("Количество отслеживаемых человек: {}", count))
                .await?;
            }
            ()
        }
        Command::SetCount(cnt) => 
        {
            if is_reject(Arc::clone(&bot),msg.chat.id, Arc::clone(&state)).await
            {
                return Ok(());
            }
            let mut guard = state.users_states.write().await;
            if let Some(us) = guard.get_mut(&chat_id)
            {
                us.set_count(cnt);
            }
            drop(guard);
            state.save_users().await;
            let _sended = bot.parse_mode(ParseMode::MarkdownV2).send_message(msg.chat.id, format!("Количество отслеживаемых человек установлено на: {}", cnt))
        .await?;
        ()
        }
        Command::Status => 
        {
            if is_reject(Arc::clone(&bot),msg.chat.id, Arc::clone(&state)).await
            {
                return Ok(());
            }
            let guard = state.users_states.read().await;
            if let Some(u) = guard.get(&chat_id)
            {
                let _sended = bot.parse_mode(ParseMode::MarkdownV2).send_message(msg.chat.id, u.to_string())
                        .await?;
            }
            ()
        }
        Command::Reg(key) => 
        {
            let mut key_guard = state.keys.write().await;
            if !key_guard.register(&key, chat_id).await
            {
                let _sended = bot.parse_mode(ParseMode::MarkdownV2).send_message(msg.chat.id, "❌ *Неправильный ключ регистрации*".to_owned())
                 .await?;
                return Ok(());
            }
            else 
            {
                let _sended = bot.parse_mode(ParseMode::MarkdownV2).send_message(msg.chat.id, "✅ *Бот успешно подключен к текущему чату*".to_owned())
                 .await?;    
            }
            drop(key_guard);
        ()
        }
    };

    Ok(())
}



///возвращает true если rejected и false если ok
async fn is_reject<C: Into<Recipient>>(bot: Arc<Bot>, chat_id: C, state: Arc<AppState>) -> bool 
{
    let key_guard = state.keys.read().await;
    let r: Recipient = chat_id.into();
    let id = if let Recipient::Id(id) = r
    {
        id.0
    }
    else
    {
        0
    };
    if !key_guard.check(&id)
    {
        let sended = bot.parse_mode(ParseMode::MarkdownV2).send_message(r, "❌ *Только зарегистрированные пользователи могут пользоваться этим функционалом*".to_owned())
        .await;
        logger::debug!("{:?}", sended);
        return true;
    }
    else 
    {
        return false;
    }
}