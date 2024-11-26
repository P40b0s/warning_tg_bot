use std::sync::Arc;
use sqlx::{sqlite::SqliteRow, FromRow, Row, SqlitePool};
use utilites::Date;
use uuid::Uuid;


use crate::users::{Status, User};

// impl FromRow<'_, SqliteRow> for User
// {
//     fn from_row(row: &SqliteRow) -> sqlx::Result<Self> 
//     {
//         let id: Uuid =  row.try_get("id")?;
//         let chat_id: i64 = row.try_get("chat_id")?;
//         let chat_id: u64 = row.try_get("user_id")?;
//         Ok(Self::new(id, name, nick, updated, status))
//     }
// }
