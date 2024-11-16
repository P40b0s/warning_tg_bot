use std::sync::Arc;

use utilites::Date;

use crate::app_state::AppState;

pub async fn reset_pluses(state: Arc<AppState>, delay: u64)
{
    loop 
    {
        let time = Date::now();
        let h = time.as_naive_datetime().time().format("%H").to_string();
        if h == "03"
        {
            let mut guard = state.users_states.write().await;
            for u in guard.iter_mut()
            {
                u.1.reset_status();
            }
            drop(guard);
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
}