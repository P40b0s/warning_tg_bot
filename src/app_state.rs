use std::{collections::HashMap, sync::Arc};

use serde::{Deserialize, Serialize};

use crate::{keys::Keys, users::{UsersMap, UsersState}};


pub struct AppState
{
    pub users_states : tokio::sync::RwLock<HashMap<i64, UsersState>>,
    pub keys: tokio::sync::RwLock<Keys>
}

impl AppState
{
    pub fn new() -> Arc<Self>
    {
        Arc::new(Self::load_users())
        
    }
    pub async fn save_users(&self)
    {
        let guard = self.users_states.read().await;
        let map = UsersMap
        {
            states: guard.clone()
        };
        let _ = utilites::serialize(map, "users.json", false, utilites::Serializer::Json);
    }
    pub fn load_users() -> Self
    {

        let keys = Keys::new();
        if let Ok(s) = utilites::deserialize::<UsersMap, _>("users.json", false, utilites::Serializer::Json)
        {
            Self
            {
                users_states: tokio::sync::RwLock::new(s.states),
                keys: tokio::sync::RwLock::new(keys)
            }
        }
        else 
        {
            Self
            {
                users_states: tokio::sync::RwLock::new(HashMap::new()),
                keys: tokio::sync::RwLock::new(keys)
            }
        }
       
    }
}