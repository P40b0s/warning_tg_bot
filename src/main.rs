mod users;
mod settings;
mod app_state;
mod api_key;
mod timer;
mod keys;
mod db;
mod error;
use db::{IGroupRepository, IUserRepository};
pub use error::Error;
use std::sync::Arc;

use app_state::AppState;
use teloxide::{dispatching::dialogue::GetChatId, prelude::*, types::{ParseMode, Recipient}, utils::command::BotCommands};
use users::{Status, Group};
use utilites::Date;
extern crate utilites;

#[tokio::main]
async fn main() 
{
    logger::StructLogger::new_default();
    logger::info!("Starting command bot...");

    let bot = Bot::new(api_key::KEY);
    let app_state = AppState::new().await;
    let sleep_state = Arc::clone(&app_state);
    tokio::spawn(async {timer::reset_pluses(sleep_state, 60*30*1).await});
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
    SetCount(u32),
    #[command(description = "*Время до которого необходимо сообщись статус*")]
    SetTime(String),
    #[command(description = "*Сколько человек необходимо оплюсить?*")]
    GetCount,
    #[command(description = "*Показать текущий статус*")]
    Status,
    #[command(description = "*Настройки оповещения*")]
    Settings,
    #[command(description = "*Чтобы начать пользоваться ботом, необходимо выполнить эту команду с выданым ключом*")]
    Reg(String),
    #[command(description = "*Посмотреть какие вводные принимает бот кроме команд*")]
    Text,
}
async fn text_handler(bot: Bot, msg: Message, state: Arc<AppState>) -> ResponseResult<()> 
{
    let bot = Arc::new(bot);
    match msg.text()
    {
        Some(text) => 
        {
            if !is_authorized(Arc::clone(&bot),msg.chat.id, Arc::clone(&state)).await
            {
                return Ok(());
            }
            let date = msg.date.clone().naive_local();
            let date  = Date::from(date);
            //change utc timezone to +3 timezone
            let date = date.add_minutes(3*60);
            match text
            {
                "+" => 
                {
                    if let Some(user) = msg.from.as_ref()
                    {
                      
                        logger::debug!("message time:{:?} user: {:?}", &date, user);
                        //let photos = bot.get_user_profile_photos(user.id).await.unwrap();
                        //let p = photos.photos.first().unwrap();
                        //let photo = bot.get_file(p.first().as_ref().unwrap().file.id.clone()).await.unwrap();
                        //logger::debug!("{:?}", photo);
                        //let fl = bot.get_file(photo.id).await.unwrap();
                        if let Some(sig) =  msg.author_signature()
                        {
                            logger::debug!("signature:{:?}", sig);
                        }
                        let user = users::User::new(user.id.0, user.first_name.clone(), user.username.clone(), date, Status::Plus);
                        if let Ok(usr) = state.repository.add_user(&user, msg.chat.id.0).await
                        {
                            logger::debug!("state:{:?}", &usr);
                            bot.parse_mode(ParseMode::MarkdownV2).send_message(msg.chat.id, usr.to_string())
                            .await?;
                        }
                    };
                    ()
                },
                "-" => 
                {
                    if let Some(user) = msg.from.as_ref()
                    {
                        let user = users::User::new(user.id.0, user.first_name.clone(), user.username.clone(), date, Status::Minus);
                        if let Ok(usr) = state.repository.add_user(&user, msg.chat.id.0).await
                        {
                            logger::debug!("state:{:?}", &usr);
                            bot.parse_mode(ParseMode::MarkdownV2).send_message(msg.chat.id, usr.to_string())
                            .await?;
                        }
                    };
                    ()
                },
                "--" => 
                {
                    if let Some(user) = msg.from.as_ref()
                    {
                        //let user = users::User::new(user.id.0, user.first_name.clone(), user.username.clone(), date, Status::Minus);
                        if let Ok(_) = state.repository.groups_repository.remove_user_from_chat(msg.chat.id.0, user.id.0).await
                        {
                            let users_state = state.repository.groups_repository.get_group(msg.chat.id.0).await;
                            if let Ok(st) = users_state
                            {
                                let _sended = bot.parse_mode(ParseMode::MarkdownV2).send_message(msg.chat.id, st.to_string())
                                .await?;
                            }
                        }
                    };
                    ()
                },
                "!" => 
                {
                    if let Some(user) = msg.from.as_ref()
                    {
                        let r = Recipient::Id(ChatId(user.id.0 as i64));
                        bot.parse_mode(ParseMode::MarkdownV2).send_message(r, "ШШШ тебе я прошепчу")
                        .await?;
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
            if !is_authorized(Arc::clone(&bot),msg.chat.id, Arc::clone(&state)).await
            {
                return Ok(());
            }
            let count = state.repository.groups_repository.get_users_count(chat_id).await;
            if let Ok(count) = count
            {
                let _sended = bot.parse_mode(ParseMode::MarkdownV2).send_message(msg.chat.id, format!("Количество отслеживаемых человек: {}", count))
                .await?;
            }
            ()
        }
        Command::SetCount(count) => 
        {
            if !is_authorized(Arc::clone(&bot),msg.chat.id, Arc::clone(&state)).await
            {
                return Ok(());
            }
            if let Ok(_) = state.repository.groups_repository.set_users_count(chat_id, count).await
            {
                let _sended = bot.parse_mode(ParseMode::MarkdownV2).send_message(msg.chat.id, format!("Количество отслеживаемых человек установлено на: {}", count))
                .await?;
            }
           
        ()
        },
        Command::SetTime(time) => 
        {
            if !is_authorized(Arc::clone(&bot),msg.chat.id, Arc::clone(&state)).await
            {
                return Ok(());
            }
            if let Some(t) = Date::parse(time)
            {
                
                let _sended = bot.parse_mode(ParseMode::MarkdownV2).send_message(msg.chat.id, format!("Количество отслеживаемых человек установлено на: {}", count))
                .await?;
            }
            else
            {
                let _sended = bot.parse_mode(ParseMode::MarkdownV2).send_message(msg.chat.id, ["❌ ", "*Ошибка формата времени, время должно быть указано в формате 23:00:00*"].concat())
                .await?;
               return Ok(());  
            }
           
        ()
        }
        Command::Status => 
        {
            if !is_authorized(Arc::clone(&bot),msg.chat.id, Arc::clone(&state)).await
            {
                return Ok(());
            }
            let users_state = state.repository.groups_repository.get_group(chat_id).await;
            if let Ok(st) = users_state
            {
                let _sended = bot.parse_mode(ParseMode::MarkdownV2).send_message(msg.chat.id, st.to_string())
                .await?;
            }
            ()
        },
        Command::Settings => 
        {
            if !is_authorized(Arc::clone(&bot),msg.chat.id, Arc::clone(&state)).await
            {
                return Ok(());
            }
            let settings = state.repository.groups_repository.get_group_settings(chat_id).await;
            if let Ok(st) = settings
            {
                let _sended = bot.parse_mode(ParseMode::MarkdownV2).send_message(msg.chat.id, st.to_string())
                .await?;
            }
            ()
        }
        Command::Reg(key) => 
        {
            let reg_result = state.repository.groups_repository.register_group(chat_id, &key).await;
            if let Ok(_) = reg_result
            {
                let _sended = bot.parse_mode(ParseMode::MarkdownV2).send_message(msg.chat.id, "✅ *Функционал бота теперь доступен в текущем чате*".to_owned())
                .await?;  
            }
            else 
            {
                let _sended = bot.parse_mode(ParseMode::MarkdownV2).send_message(msg.chat.id, ["❌ ", "*", &reg_result.err().unwrap().to_string(), "*"].concat())
                .await?;
               return Ok(());  
            }
        ()
        },
        Command::Text => 
        {
           
            let mut lines = String::new();

            lines.push_str([&teloxide::utils::markdown::escape("+ "), "*Добавляет человека к списку отслеживаемых или обновляет его статус на ✅*\n"].concat().as_str());
            lines.push_str([&teloxide::utils::markdown::escape("- "), "*Меняет статус человека в списке отслеживаемых на ❌*\n"].concat().as_str());
            lines.push_str([&teloxide::utils::markdown::escape("-- "), "*Удаляет человека из списка отслеживаемых в данной группе*\n"].concat().as_str());
            lines.push_str("*Каждые сутки в 3 часа ночи происхлдит общий сброс статусов, поэтому если необходимо отметиться нужно обновить свой статус способом указанным выше*\n");
            let _sended = bot.parse_mode(ParseMode::MarkdownV2).send_message(msg.chat.id, lines)
            .await?;
            ()
        }

    };

    Ok(())
}



/// Current chat id is is authorized for using bot functions (or not)
async fn is_authorized<C: Into<Recipient>>(bot: Arc<Bot>, chat_id: C, state: Arc<AppState>) -> bool 
{
    let r: Recipient = chat_id.into();
    let id = if let Recipient::Id(id) = r
    {
        id.0
    }
    else
    {
        0
    };
    let reg = state.repository.groups_repository.chat_is_authorized(id).await;
    if let Ok(reg) = reg
    {
        if !reg
        {
            let sended = bot.parse_mode(ParseMode::MarkdownV2).send_message(r, "❌ *Функции бота можно использовать только в зарегистрированных группах, необходимо зарегистрировать группу командой /reg ваш_ключ_доступа*".to_owned())
            .await;
            logger::debug!("{:?}", sended);
            return false;
        }
        else 
        {
            return true;
        }
    }
    else 
    {
        logger::error!("{}", reg.err().unwrap());
        return false;
    }
}