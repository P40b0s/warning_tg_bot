use std::collections::HashSet;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Keys(HashSet<i64>);
const KEY: &str = "713e4412-1962-47d6-9ae3-9d658b1a06c7";
impl Keys
{
    pub fn new() -> Self
    {
        Self::load()
    }
    async fn save(&self)
    {
        let _s = utilites::serialize(self, "authorized.json", false, utilites::Serializer::Json);
        logger::info!("{:?}", _s);
    }
    fn load() -> Self
    {

        if let Ok(s) = utilites::deserialize("authorized.json", false, utilites::Serializer::Json)
        {
            s
        }
        else 
        {
            Self(HashSet::new())
        }
    }
    pub async fn register(&mut self, key: &str, chat_id: i64) -> bool
    {
        if key == KEY
        {
            self.0.insert(chat_id);
            self.save().await;
            return true;
        }
        else 
        {
            return false;
        }
    }
    pub fn check(&self, chat_id: &i64) -> bool
    {
        self.0.contains(chat_id)
    }
}

#[cfg(test)]
mod tests
{
    use utilites::Date;
   
}