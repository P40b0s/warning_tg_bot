use std::sync::Arc;

use utilites::Date;

use crate::{app_state::AppState, db::IUserRepository};

pub async fn reset_pluses(state: Arc<AppState>, delay: u64)
{
    loop 
    {
        let time = Date::now();
        let settings = 
      
        if Date::time_in_range(&time, (3, 30, 0), (3, 59, 59))
        {
            let _ = state.repository.users_repository.set_status_for_all(crate::users::Status::Minus).await;
        }
        if Date::time_in_range(&time, (3, 30, 0), (3, 59, 59))
        {
            let _ = state.repository.users_repository.set_status_for_all(crate::users::Status::Minus).await;
        }
        tokio::time::sleep(tokio::time::Duration::from_secs(delay)).await;
    }
}

#[cfg(test)]
mod tests
{
    use utilites::Date;

    #[test]
    fn test_hours()
    {
        logger::StructLogger::new_default();
        let time = Date::now().add_minutes(6*60);
        let h = time.as_naive_datetime().time().format("%H").to_string();
        logger::info!("h: {}", h);
    }

    #[test]
    fn test_hours2()
    {
        logger::StructLogger::new_default();
        let time = Date::now();
        if Date::time_in_range(&time, (20, 0, 0), (20, 59, 59))
        {
            logger::info!("h: {}", time.to_string());
        }
    }
    #[test]
    fn test_hours3()
    {
        logger::StructLogger::new_default();
        let time = Date::now();
        if time.time_in_hour(1)
        {
            logger::info!("h: {}", time.to_string());
        }
    }
}