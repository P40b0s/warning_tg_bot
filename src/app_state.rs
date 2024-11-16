use std::sync::Arc;

use crate::{settings::Settings, users::UsersState};

pub struct AppState
{
    pub users_state : tokio::sync::RwLock<UsersState>,
    pub settings: tokio::sync::RwLock<Settings>
}
impl AppState
{
    pub fn new() -> Arc<Self>
    {
        let settings = Settings::load();
        let users_state: UsersState = settings.clone().into();
        Arc::new(Self
        {
            users_state: tokio::sync::RwLock::new(users_state),
            settings: tokio::sync::RwLock::new(settings)
        })
    }
}