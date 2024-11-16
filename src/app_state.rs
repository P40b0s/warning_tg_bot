use std::{collections::HashMap, sync::Arc};

use crate::{settings::{Settings, SettingsMap}, users::UsersState};

pub struct AppState
{
    pub users_states : HashMap<i64, tokio::sync::RwLock<UsersState>>,
    pub settings: HashMap<i64, tokio::sync::RwLock<Settings>>
}
//TODO нужен hashmap с настройками и hashmap  с UserState для каждого чата нужно свое состояние
impl AppState
{
    pub fn new() -> Arc<Self>
    {
        let settings = SettingsMap::default();
        
        let users_state: UsersState = settings.clone().into();

        Arc::new(Self
        {
            users_states: HashMap::tokio::sync::RwLock::new(users_state),
            settings: tokio::sync::RwLock::new(settings)
        })
    }
}