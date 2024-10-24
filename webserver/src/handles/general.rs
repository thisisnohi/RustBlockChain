use actix_web::{web, HttpResponse};

use crate::state::AppState;

pub async fn health_check_handler(app_state: web::Data<AppState>) -> HttpResponse {
    let health_check_response = &app_state.health_check_response;
    let mut visit_count = app_state.visit_count.lock().unwrap();

    let response = format!(
        "{} {} times, 可以操作数据库，返回数据信息，以判断是否正常 ",
        health_check_response, visit_count
    );

    *visit_count += 1;

    // 返回
    HttpResponse::Ok().json(&response)
}
