use std::collections::HashMap;

use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Clone)]
pub struct Settings
{
    pub count: u8
}
#[derive(Serialize, Deserialize, Clone)]
pub struct SettingsMap
{
    settings: HashMap<i64, Settings>
}

impl Default for Settings
{
    fn default() -> Self 
    {
        Self
        {
            count: 0
        }    
    }
}
const FILE_NAME: &str = "settings.toml";
impl Settings
{
    pub fn load() -> Self
    {
        if let Ok(s) = utilites::deserialize(FILE_NAME, false, utilites::Serializer::Toml)
        {
            s
        }
        else 
        {
            Self::default()    
        }
    }
    pub fn save(&self)
    {
        utilites::serialize(self, FILE_NAME, false, utilites::Serializer::Toml);
    }
}


impl Default for SettingsMap
{
    fn default() -> Self 
    {
        let mut hm = HashMap::new();
        let settings = Settings::default();
        hm.insert(0, settings);
        Self
        {
            settings: hm
        }
    }
}