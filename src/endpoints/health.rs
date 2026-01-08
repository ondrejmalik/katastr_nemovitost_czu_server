use deadpool_postgres::Pool;
use axum::extract::State;

pub async fn get_health(State(_pool): State<Pool>) -> Result<(), String> {
    Ok(())
}
