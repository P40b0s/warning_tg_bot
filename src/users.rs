use std::{collections::HashMap, fmt::format};

use serde::{Deserialize, Serialize};
use utilites::Date;



#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UsersState
{
    users: Vec<State>,
    count: u8,
    last_position_index: usize
}
impl Default for UsersState 
{
    fn default() -> Self 
    {
        Self
        {
            users: Vec::with_capacity(0),
            count: 0,
            last_position_index: 0
        }
    }   
}


impl UsersState
{
    pub fn add_of_replace_status(&mut self, state: State)
    {
        let user = self.users.iter_mut().find(|s| *s == &state);
        if let Some(user) = user
        {
            *user = state
        }
        else 
        {
            //пока индекс меньше количества, заменяем дефолтные стейты, как только сравняется начинаем добавлять новых
            if self.users.len() > self.last_position_index
            {
                self.users[self.last_position_index] = state;
                self.last_position_index +=1;
            }
            else 
            {
                self.users.push(state);
                self.count +=1;
            }
        }
    }
    pub fn clear(&mut self)
    {
        *self = UsersState::default();
    }
    pub fn set_count(&mut self, count: u8)
    {
        if self.count > 0 && self.count > count
        {
            let del_count = self.count - count;
            for _ in 0.. del_count
            {
                self.users.pop();
            }
            self.count = count;
            return;
        }
        if self.count == 0
        {
            self.count = count;
            let count: usize = count.into(); 
            self.users = Vec::with_capacity(count);
            for _ in 0.. count
            {
                self.users.push(State::default());
            }
            return;
        }
        let add_count = count - self.count;
        for _ in 0..add_count
        {
            self.users.push(State::default());
        }
        self.count = count;
    }
    pub fn get_count(&self) -> u8
    {
        self.count
    }
    pub fn reset_status(&mut self)
    {
        for u in self.users.iter_mut()
        {
            u.change_status(Status::Minus);
        }
    }
    pub fn get_process(current_count: usize, overall_count: u8) -> String
    {
        if current_count == 0
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
    id: u64,
    name: String,
    nick: Option<String>,
    updated: Date,
}
impl PartialEq for User
{
    fn eq(&self, other: &Self) -> bool 
    {
        &self.id == &other.id
    }
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct State
{
    user: Option<User>,
    status: Status
}
impl PartialEq for State
{
    fn eq(&self, other: &Self) -> bool 
    {
        if self.user.is_none() || other.user.is_none()
        {
            return false;
        }
        let u1 = self.user.as_ref().unwrap();
        let u2 = other.user.as_ref().unwrap();
        u1 == u2
    }
}

impl Default for State
{
    fn default() -> Self 
    {
        Self
        {
            user: None,
            status: Status::Minus
        }
    }
}
impl State
{
    pub fn new(user: Option<User>) -> Self
    {
        if let Some(user) = user
        {
            Self
            {
                user: Some(user),
                status: Status::Plus
            }
        }
        else 
        {
            Self::default()    

        }
    }
    pub fn change_status(&mut self, status: Status)
    {
        self.status = status;
        if let Some(u) = self.user.as_mut()
        {
            u.updated = Date::now();
        }
    }
}
impl ToString for UsersState
{
    fn to_string(&self) -> String 
    {
        let mut output = String::new();
        let plus_count = self.users.iter().filter(|f| f.status == Status::Plus).count();
        output.push_str(&format!("*Статус {}/{}*\n", plus_count, self.count));
        output.push_str(&[Self::get_process(plus_count, self.count), "\n".to_owned()].concat());
        output.push_str(&["—".repeat(16), "\n".to_owned()].concat());
        for s in &self.users
        {
            if let Some(u) = s.user.as_ref()
            {
                let nick = match u.nick.as_ref()
                {
                    Some(n) => format!("\\([{}](tg://user?id={})\\)",teloxide::utils::markdown::escape(&n), u.id),
                    None => "".to_owned()
                };
                let check = match s.status
                {
                    Status::Minus => "❌",
                    Status::Plus => "✅"
                };
                let date = u.updated.format(utilites::DateFormat::Serialize);
                let date = date.split("T").collect::<Vec<_>>();
                let line = format!("{} {} {}\n🕛 {} {}\n",check, teloxide::utils::markdown::escape(&u.name), nick, date[0].replace("-", "\\."), date[1]);
                output.push_str(&line);
                output.push_str(&["—".repeat(16), "\n".to_owned()].concat());
            }
            else 
            {
                let line = "❌ Неизвестно\n";
                output.push_str(line);
                output.push_str(&["—".repeat(16), "\n".to_owned()].concat());
            }
           
        }
        output
    }
    
}

impl User
{
    pub fn new(id: u64, name: String, nick: Option<String>, date: Date) -> Self
    {
        let date = date.add_minutes(3 * 60);
        Self 
        {
            id,
            name,
            nick,
            updated: date
        }
    }
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct  UsersMap
{
    pub states: HashMap<i64, UsersState>
}


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