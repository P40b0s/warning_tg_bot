use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Clone)]
pub struct Settings
{
    pub count: u8
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