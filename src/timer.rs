use std::sync::Arc;

use utilites::Date;

use crate::app_state::AppState;

pub async fn reset_pluses(state: Arc<AppState>, delay: u64)
{
    logger::info!("стфрт лупера{}", Date::now());
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
            logger::info!("Статус юзеров сброшен {}", Date::now())
        }
        let mut guard = state.users_states.write().await;
        for u in guard.iter_mut()
        {
            u.1.reset_status();
        }
        drop(guard);
        logger::info!("Статус юзеров сброшен {}", Date::now());
        tokio::time::sleep(tokio::time::Duration::from_secs(delay)).await;
    }
}