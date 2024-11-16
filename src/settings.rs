// use std::collections::HashMap;

// use serde::{Deserialize, Serialize};

// const FILE_NAME: &str = "settings.toml";
// #[derive(Serialize, Deserialize, Clone)]
// pub struct Settings
// {
//     pub count: u8
// }
// #[derive(Serialize, Deserialize, Clone)]
// pub struct SettingsMap
// {
//     pub settings: HashMap<i64, Settings>
// }
// impl SettingsMap
// {
//     pub fn load() -> Self
//     {
//         if let Ok(s) = utilites::deserialize(FILE_NAME, false, utilites::Serializer::Toml)
//         {
//             s
//         }
//         else 
//         {
//             Self
//             {
//                 settings: HashMap::new()
//             }
//         }
//     }
//     pub fn save(&self)
//     {
//         utilites::serialize(self, FILE_NAME, false, utilites::Serializer::Toml);
//     }
// }

// impl Default for Settings
// {
//     fn default() -> Self 
//     {
//         Self
//         {
//             count: 0
//         }    
//     }
// }
// #[cfg(test)]
// mod tests
// {
//     #[test]
//     pub fn test_settings()
//     {
//         let sm = super::SettingsMap::load();
//         sm.save();
//     }
// }