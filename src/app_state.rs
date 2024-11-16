use std::{collections::HashMap, sync::Arc};

use serde::{Deserialize, Serialize};

use crate::users::{UsersMap, UsersState};


pub struct AppState
{
    pub users_states : tokio::sync::RwLock<HashMap<i64, UsersState>>,
    //pub settings: tokio::sync::RwLock<SettingsMap>
}

impl AppState
{
    pub fn new() -> Arc<Self>
    {
        //let settings = SettingsMap::load();
        Arc::new(Self::load_users())
        
    }
    pub async fn save_users(&self)
    {
        let guard = self.users_states.read().await;
        let map = UsersMap
        {
            states: guard.clone()
        };
        let r = utilites::serialize(map, "users.json", false, utilites::Serializer::Json);
    }
    pub fn load_users() -> Self
    {

        if let Ok(s) = utilites::deserialize::<UsersMap, _>("users.json", false, utilites::Serializer::Json)
        {
            Self
            {
                users_states: tokio::sync::RwLock::new(s.states)
            }
        }
        else 
        {
            Self
            {
                users_states: tokio::sync::RwLock::new(HashMap::new()),
                //settings: tokio::sync::RwLock::new(settings)
            }
        }
       
    }
}