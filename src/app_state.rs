use std::{collections::HashMap, sync::Arc};

use serde::{Deserialize, Serialize};

use crate::{db::{new_connection, GroupRepository, IGroupRepository, IUserRepository, UserRepository}, keys::Keys, users::{UsersMap, UsersState}};


pub struct AppState
{
    pub groups_repository: GroupRepository,
    pub users_repository: UserRepository
}

impl AppState
{
    pub async fn new() -> Arc<Self>
    {
        let pool = Arc::new(new_connection("bot").await.unwrap());
        Arc::new(
            Self
            {
                groups_repository: GroupRepository::new(Arc::clone(&pool)),
                users_repository: UserRepository::new(pool)
            }
        )
    }
    // pub async fn save_users(&self)
    // {
    //     let guard = self.users_states.read().await;
    //     let map = UsersMap
    //     {
    //         states: guard.clone()
    //     };
    //     let _ = utilites::serialize(map, "users.json", false, utilites::Serializer::Json);
    // }
    // pub fn load_users() -> Self
    // {

    //     let keys = Keys::new();
    //     if let Ok(s) = utilites::deserialize::<UsersMap, _>("users.json", false, utilites::Serializer::Json)
    //     {
    //         Self
    //         {
    //             users_states: tokio::sync::RwLock::new(s.states),
    //             keys: tokio::sync::RwLock::new(keys)
    //         }
    //     }
    //     else 
    //     {
    //         Self
    //         {
    //             users_states: tokio::sync::RwLock::new(HashMap::new()),
    //             keys: tokio::sync::RwLock::new(keys)
    //         }
    //     }
       
    // }
}