use std::fmt::format;

use utilites::Date;

use crate::settings::Settings;

#[derive(Debug)]
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
impl Into<UsersState> for Settings
{
    fn into(self) -> UsersState 
    {
        UsersState
        {
            count: self.count,
            ..Default::default()
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
        if self.count > 0 && self.count < count
        {
            let del_count = count - self.count;
            for _ in 0.. del_count
            {
                self.users.pop();
            }
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
        }
    }
    pub fn get_count(&self) -> u8
    {
        self.count
    }
}
#[derive(Debug)]
pub enum Status
{
    Plus,
    Minus
}

#[derive(Debug)]
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
#[derive(Debug)]
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
    }
}
impl ToString for UsersState
{
    fn to_string(&self) -> String 
    {
        let mut output = String::new();
        for s in &self.users
        {
            if let Some(u) = s.user.as_ref()
            {
                let nick = match u.nick.as_ref()
                {
                    Some(n) => [" (", n, ") "].concat(),
                    None => "".to_owned()
                };
                let check = match s.status
                {
                    Status::Minus => "❌",
                    Status::Plus => "✅"
                };
                let date = u.updated.format(utilites::DateFormat::Serialize);
                let date = date.split("T").collect::<Vec<_>>();
                let line = format!("{}{} {}\n{} {}\n",check, u.name, nick, date[0], date[1]);
                output.push_str(&line);
            }
            else 
            {
                let line = "❌ Неизвестно\n";
                output.push_str(line);
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