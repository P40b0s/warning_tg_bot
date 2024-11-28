use std::{collections::HashMap, fmt::format};

use serde::{Deserialize, Serialize};
use utilites::Date;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GroupSettings
{
    pub chat_id: i64,
    pub users_count: u32,
    pub deadline_time: Date,
    pub additional_dates: Vec<Date>,
    pub group_name: Option<String>,
    pub is_active: bool
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Group
{
    chat_id: i64,
    users: Vec<User>,
    settings: Option<GroupSettings>,
}

impl Group
{
    pub fn new(chat_id: i64, users: Vec<User>) -> Self
    {
        Self
        {
            chat_id,
            users,
            settings: None
        }
    }
    pub fn add_settings(self, settings: GroupSettings) -> Self
    {
        Self
        {
            settings : Some(settings),
            ..self
        }
    }
    pub fn get_settings<'a>(&'a self) -> Option<&'a GroupSettings>
    {
        self.settings.as_ref()
    }
    
    pub fn get_process(current_count: usize, overall_count: u32) -> String
    {
        if current_count == 0 || overall_count == 0
        {
            return "🟥".repeat(10);
        }
        let percent: u32 = ((current_count as f32 / overall_count as f32) * 10.0) as u32;
        let red_count = 10 - percent;
        ["🟩".repeat(percent as usize), "🟥".repeat(red_count as usize), (percent*10).to_string(), "%".to_owned()].concat()
    }
}
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum Status
{
    Plus,
    Minus
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct User
{
    pub id: u64,
    pub username: String,
    pub nick: Option<String>,
    pub updated: Date,
    pub current_status: Status
}
impl PartialEq for User
{
    fn eq(&self, other: &Self) -> bool 
    {
        &self.id == &other.id
    }
}
// #[derive(Serialize, Deserialize, Clone, Debug)]
// pub struct State
// {
//     user: User,
//     status: Status
// }
// impl PartialEq for State
// {
//     fn eq(&self, other: &Self) -> bool 
//     {
//         if self.user.is_none() || other.user.is_none()
//         {
//             return false;
//         }
//         let u1 = self.user.as_ref().unwrap();
//         let u2 = other.user.as_ref().unwrap();
//         u1 == u2
//     }
// }

// impl Default for State
// {
//     fn default() -> Self 
//     {
//         Self
//         {
//             user: None,
//             status: Status::Minus
//         }
//     }
// }
impl User
{
    pub fn new(id: u64, username: String, nick: Option<String>, date: Date, current_status: Status) -> Self
    {
        //let date = date.add_minutes(3 * 60);
        Self 
        {
            id,
            username,
            nick,
            updated: date,
            current_status
        }
    }
    pub fn change_status(&mut self, status: Status)
    {
        self.current_status = status;
        self.updated = Date::now();
    }
}
// impl State
// {
//     pub fn new(user: Option<User>) -> Self
//     {
//         if let Some(user) = user
//         {
//             Self
//             {
//                 user: Some(user),
//                 status: Status::Plus
//             }
//         }
//         else 
//         {
//             Self::default()    

//         }
//     }
//     pub fn change_status(&mut self, status: Status)
//     {
//         self.status = status;
//         if let Some(u) = self.user.as_mut()
//         {
//             u.updated = Date::now();
//         }
//     }
// }
impl ToString for Group
{
    fn to_string(&self) -> String 
    {
        let mut output = String::new();
        let plus_count = self.users.iter().filter(|f| f.current_status == Status::Plus).count();
        let settings = self.get_settings();
        let count = settings.and_then(|c| Some(c.users_count)).unwrap_or_default();
        output.push_str(&format!("*Статус {}/{}*\n", plus_count, count));
        output.push_str(&[Self::get_process(plus_count, count), "\n".to_owned()].concat());
        output.push_str(&["—".repeat(16), "\n".to_owned()].concat());
        for i in  0..count as usize
        {
           if let Some(u) = self.users.get(i)
           {
                let nick = match u.nick.as_ref()
                {
                    Some(n) => format!("\\([{}](tg://user?id={})\\)",teloxide::utils::markdown::escape(&n), u.id),
                    None => "".to_owned()
                };
                let check = match u.current_status
                {
                    Status::Minus => "❌",
                    Status::Plus => "✅"
                };
                let date = u.updated.format(utilites::DateFormat::Serialize);
                let date = date.split("T").collect::<Vec<_>>();
                let line = format!("{} {} {}\n🕛 {} {}\n",check, teloxide::utils::markdown::escape(&u.username), nick, date[0].replace("-", "\\."), date[1]);
                output.push_str(&line);
                output.push_str(&["—".repeat(16), "\n".to_owned()].concat());
           }
           else 
           {
                output.push_str("❌ Неизвестный\n");
                output.push_str(&["—".repeat(16), "\n".to_owned()].concat());
           }
        }
        output
    }
    
}


impl ToString for GroupSettings
{
    fn to_string(&self) -> String 
    {
        let mut output = String::new();
        output.push_str("*📜 Настройки 📜*\n");
        output.push_str(&["—".repeat(16), "\n".to_owned()].concat());
        output.push_str(&["🐣 Количетво проверяемых: *",self.users_count.to_string().as_str(), "*\n"].concat());
        output.push_str(&["—".repeat(16), "\n".to_owned()].concat());
        output.push_str(&["⏳ Отчетное время: *",self.deadline_time.format(utilites::DateFormat::Time).as_str(), "*\n"].concat());
        output.push_str(&["—".repeat(16), "\n".to_owned()].concat());
        if self.additional_dates.len() > 0
        {
            output.push_str("*😱 Дополнительные даты оповещения 😱*\n");

            for d in &self.additional_dates
            {
                output.push_str(&["📢 ", d.format(utilites::DateFormat::DotDate).as_str(), "*\n"].concat());
            }
            output.push_str(&["—".repeat(16), "\n".to_owned()].concat());
        }
        output
    }
    
}

// impl User
// {
//     pub fn new(id: u64, name: String, nick: Option<String>, date: Date) -> Self
//     {
//         let date = date.add_minutes(3 * 60);
//         Self 
//         {
//             id,
//             name,
//             nick,
//             updated: date
//         }
//     }
// }

#[cfg(test)]
mod tests
{

    pub fn get_process(current_count: u32, overall_count: u32) -> String
    {
        if current_count == 0
        {
            //return "🟩"
            return "🟥".repeat(10);
        }
        let count = overall_count - current_count;
        let percent: u32 = ((current_count as f32 / overall_count as f32) * 10.0) as u32;
        logger::debug!("{} {}", count,  percent);
        let red_count = 10 - percent;
        ["🟩".repeat(percent as usize), "🟥".repeat(red_count as usize)].concat()
    }
    #[test]
    pub fn test_settings()
    {
        logger::StructLogger::new_default();
        logger::debug!("{}", get_process(10, 10));
    }
}