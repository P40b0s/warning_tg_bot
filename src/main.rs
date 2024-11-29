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
use teloxide::{dispatching::dialogue::GetChatId, prelude::*, types::{BotCommand, MessageId, ParseMode, ReactionType, Recipient, True}, utils::command::{BotCommands, CommandDescription}, RequestError};
use users::{Status, Group};
use utilites::Date;
extern crate utilites;

#[tokio::main]
async fn main() 
{
    logger::StructLogger::new_default();
    logger::info!("Starting command bot...");
    let bot = Bot::new(api_key::KEY);
    set_bot_commands(&bot).await;
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
enum Command 
{
    #[command(description = "Показать это сообщение")]
    Help,
    #[command(description = "Установить количество человек которое необходимо оплюсить")]
    SetCount(u32),
    #[command(description = "Время до которого необходимо сообщить статус \n вводится в формате 12:00:00")]
    SetTime(String),
    #[command(description = "Добавляет дополнительные даты в которые будет необходимо оставить свой отчет можно ввести несколько подряд, пример: 03\\.11\\.2024 04\\.11\\.2024 05\\.11\\.2024")]
    AddDates(String),
    #[command(description = "Сколько человек необходимо оплюсить?")]
    GetCount,
    #[command(description = "Показать текущий статус")]
    Status,
    #[command(description = "Настройки оповещения")]
    Settings,
    #[command(description = "Чтобы начать пользоваться ботом, необходимо выполнить эту команду с выданым ключом")]
    Reg(String),
    #[command(description = "Установить свой текущий статус `готов`")]
    Ready,
    #[command(description = "Установить свой текущий статус `не готов` ")]
    UnReady,
    #[command(description = "Удалить себя из списка отслеживаемых в этой группе")]
    Exit,
    #[command(description = "Заболевание одним словом диакгноз и через пробел дату выхода: ` ОРВИ 12.10.2024`")]
    Disease(String),
    #[command(description = "Отпуск через пробел 2 даты, начало и окончание отпуска: ` 12.10.2024 12.12.2024`")]
    Vacation(String)
}

async fn set_bot_commands(bot: &Bot)
{
    // match Command
    // {
    //     Command::Text => 
    //     {
    //         Command::bot_commands()
    //     }
    // }

    let _ = bot.set_my_commands(Command::bot_commands()).await;
    //bot.set_my_commands(vec![BotCommand { command: "help".to_string(), description: "Помощь".to_string()}]).await;
}
// help - Показать это сообщение
// setcount - Установить количество человек которое необходимо оплюсить
// settime - Время до которого необходимо сообщись статус  вводится в формате 12:00:00
// adddates - Добавляет дополнительные даты в которые будет необходимо оставить свой отчет можно ввести несколько подряд, пример: 03.11.2024 04.11.2024 05.11.2024
// getcount - Сколько человек необходимо оплюсить?
// status - Показать текущий статус
// settings - Настройки оповещения
// reg - Чтобы начать пользоваться ботом, необходимо выполнить эту команду с выданым ключом
// text - Посмотреть какие вводные принимает бот кроме команд
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
                // "+" => 
                // {
                //     if let Some(user) = msg.from.as_ref()
                //     {
                      
                //         logger::debug!("message time:{:?} user: {:?}", &date, user);
                //         //let photos = bot.get_user_profile_photos(user.id).await.unwrap();
                //         //let p = photos.photos.first().unwrap();
                //         //let photo = bot.get_file(p.first().as_ref().unwrap().file.id.clone()).await.unwrap();
                //         //logger::debug!("{:?}", photo);
                //         //let fl = bot.get_file(photo.id).await.unwrap();
                //         if let Some(sig) =  msg.author_signature()
                //         {
                //             logger::debug!("signature:{:?}", sig);
                //         }
                //         let user = users::User::new(user.id.0, user.first_name.clone(), user.username.clone(), date, Status::Plus);
                //         if let Ok(usr) = state.repository.add_user(&user, msg.chat.id.0).await
                //         {
                //             logger::debug!("state:{:?}", &usr);
                //             bot.parse_mode(ParseMode::MarkdownV2).send_message(msg.chat.id, usr.to_string())
                //             .await?;
                //         }
                //     };
                //     ()
                // },
                // "-" => 
                // {
                //     if let Some(user) = msg.from.as_ref()
                //     {
                //         let user = users::User::new(user.id.0, user.first_name.clone(), user.username.clone(), date, Status::Minus);
                //         if let Ok(usr) = state.repository.add_user(&user, msg.chat.id.0).await
                //         {
                //             logger::debug!("state:{:?}", &usr);
                //             // bot.set_chat_menu_button()
                //             // .chat_id(msg.chat.id)
                //             // .menu_button(teloxide::types::MenuButton::Commands)
                //             // .await?;
                //             bot.parse_mode(ParseMode::MarkdownV2).send_message(msg.chat.id, usr.to_string())
                //             .await?;
                //         }
                //     };
                //     ()
                // },
                // "--" => 
                // {
                //     if let Some(user) = msg.from.as_ref()
                //     {
                //         //let user = users::User::new(user.id.0, user.first_name.clone(), user.username.clone(), date, Status::Minus);
                //         if let Ok(_) = state.repository.groups_repository.remove_user_from_chat(msg.chat.id.0, user.id.0).await
                //         {
                //             let users_state = state.repository.groups_repository.get_group(msg.chat.id.0).await;
                //             if let Ok(st) = users_state
                //             {
                //                 let _sended = bot.parse_mode(ParseMode::MarkdownV2).send_message(msg.chat.id, st.to_string())
                //                 .await?;
                //             }
                //         }
                //     };
                //     ()
                // },
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
    let date = msg.date.clone().naive_local();
    let date  = Date::from(date);
    //change utc timezone to +3 timezone
    let date = date.add_minutes(3*60);
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
                let time = t.format(utilites::DateFormat::Time);
                let _ = state.repository.groups_repository.set_deadline_time(chat_id, t).await;
                let _sended = bot.parse_mode(ParseMode::MarkdownV2).send_message(msg.chat.id, format!("Время установлено на: {}", time))
                .await?;
            }
            else
            {
                let _sended = bot.parse_mode(ParseMode::MarkdownV2).send_message(msg.chat.id, ["❌ ", "*Ошибка формата времени, время должно быть указано в формате 23:00:00*"].concat())
                .await?;
               return Ok(());  
            }
           
        ()
        },
        Command::AddDates(dates) => 
        {
            if !is_authorized(Arc::clone(&bot),msg.chat.id, Arc::clone(&state)).await
            {
                return Ok(());
            }
            let splitted: Vec<Date> = dates.split(" ").filter_map(|d| Date::parse(d)).collect();
            logger::info!("Утановлены даты: {:?}", &splitted);
            let _ = state.repository.groups_repository.set_additional_dates(chat_id, splitted).await;
            let settings = state.repository.groups_repository.get_group_settings(chat_id).await;
            if let Ok(st) = settings
            {
                let _sended = bot.parse_mode(ParseMode::MarkdownV2).send_message(msg.chat.id, st.to_string())
                .await?;
            }
        ()
        },
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
        Command::Ready =>
        {
            if !is_authorized(Arc::clone(&bot),msg.chat.id, Arc::clone(&state)).await
            {
                return Ok(());
            }
            if let Some(user) = msg.from.as_ref()
            {
                let user = users::User::new(user.id.0, user.first_name.clone(), user.username.clone(), date, Status::Plus);
                if let Ok(_) = state.repository.add_user(&user, msg.chat.id.0).await
                {
                    send_reaction(Arc::clone(&bot), "👍", id.as_ref().cloned().unwrap(), msg.id).await?;
                }
            };
        },
        Command::UnReady =>
        {
            if !is_authorized(Arc::clone(&bot),msg.chat.id, Arc::clone(&state)).await
            {
                return Ok(());
            }
            if let Some(user) = msg.from.as_ref()
            {
                let user = users::User::new(user.id.0, user.first_name.clone(), user.username.clone(), date, Status::Minus);
                if let Ok(_) = state.repository.add_user(&user, msg.chat.id.0).await
                {
                    let _ = send_reaction(Arc::clone(&bot), "😢", id.as_ref().cloned().unwrap(), msg.id).await?;
                }
            };
        },
        Command::Exit =>
        {
            if !is_authorized(Arc::clone(&bot),msg.chat.id, Arc::clone(&state)).await
            {
                return Ok(());
            }
            if let Some(user) = msg.from.as_ref()
            {
                if let Ok(_) = state.repository.groups_repository.remove_user_from_chat(msg.chat.id.0, user.id.0).await
                {
                    send_reaction(Arc::clone(&bot), "🤔", id.as_ref().cloned().unwrap(), msg.id).await?;
                }
            };
        },
        Command::Disease(dis) =>
        {
            if !is_authorized(Arc::clone(&bot),msg.chat.id, Arc::clone(&state)).await
            {
                return Ok(());
            }
           
            if let Some(user) = msg.from.as_ref()
            {
                let splitted: Vec<&str> = dis.split(' ').collect();
                if splitted.len() != 2
                {
                    bot.parse_mode(ParseMode::MarkdownV2).send_message(msg.chat.id, format!("Неправильно переданы параметры команды: {}", &dis))
                    .await?;
                }
                else 
                {
                    let dis_date = Date::parse(splitted[1]);
                    if let Some(dd) = dis_date  
                    {
                        let dis_status = Status::Disease(splitted[0].to_owned(), dd);
                        let user = users::User::new(user.id.0, user.first_name.clone(), user.username.clone(), date, dis_status);
                        if let Ok(_) = state.repository.add_user(&user, msg.chat.id.0).await
                        {
                            send_reaction(Arc::clone(&bot), "💊", id.as_ref().cloned().unwrap(), msg.id).await;
                        }
                    }
                    else 
                    {
                        bot.parse_mode(ParseMode::MarkdownV2).send_message(msg.chat.id, format!("Неправильно переданы параметры команды: {}", &dis))
                        .await?;
                    }
                }
            };
        },
        Command::Vacation(vac) =>
        {
            if !is_authorized(Arc::clone(&bot),msg.chat.id, Arc::clone(&state)).await
            {
                return Ok(());
            }
           
            if let Some(user) = msg.from.as_ref()
            {
                let mut splitted: Vec<Date> = vac.split(' ').filter_map(|d| Date::parse(d)).collect();
                if splitted.len() != 2
                {
                    bot.parse_mode(ParseMode::MarkdownV2).send_message(msg.chat.id, format!("Неправильно переданы параметры команды: {}", &vac))
                    .await?;
                }
                else 
                {
                    let date_1 = splitted.swap_remove(0);
                    let date_2 = splitted.swap_remove(0);
                    let status = Status::Vacation(date_1, date_2);
                    let user = users::User::new(user.id.0, user.first_name.clone(), user.username.clone(), date, status);
                    if let Ok(_) = state.repository.add_user(&user, msg.chat.id.0).await
                    {
                        send_reaction(Arc::clone(&bot), "🎉", id.as_ref().cloned().unwrap(), msg.id).await;
                    }
                }
            };
        },
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

/// Send reaction to message "👀"
async fn send_reaction<C: Into<Recipient>>(bot: Arc<Bot>, reaction: &str, chat_id: C, message_id: MessageId) -> Result<True, RequestError>
{
    let eyes = ReactionType::Emoji 
    {
        emoji: reaction.to_owned(),
    };
    let mut reaction = bot.set_message_reaction(chat_id, message_id);
    reaction.reaction = Some(vec![eyes]);
    reaction.send().await
}