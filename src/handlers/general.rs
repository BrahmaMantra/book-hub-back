use actix_web::{web, HttpResponse};

use crate::{error::MyError, state::AppState};

// 健康检查处理函数
pub async fn health_check_handler(
    app_state: web::Data<AppState>,
) -> Result<HttpResponse,MyError> {
    let health_check_response = &app_state.health_check_response;
    let mut visit_count = app_state.visit_count.lock().unwrap();
    let response = format!("{} {} times", health_check_response, visit_count);
    *visit_count += 1;
    Ok(HttpResponse::Ok().json(&response))
}