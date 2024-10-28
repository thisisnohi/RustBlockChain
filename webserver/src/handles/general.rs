use crate::models::node_info::NodeHealth;
use crate::state::AppState;
use actix_web::{web, HttpResponse};

pub async fn health_check_handler(app_state: web::Data<AppState>) -> HttpResponse {
    let health_check_response = &app_state.health_check_response;
    let mut visit_count = app_state.visit_count.lock().unwrap();

    log::info!("health_check_handler....");

    let response = format!(
        "{} {} times111, 可以操作数据库，返回数据信息，以判断是否正常 ",
        health_check_response, visit_count
    );
    let health_info = NodeHealth {
        node: "".to_string(),
        health: true,
        visit_count: *visit_count,
        message: response,
    };
    log::info!("health_check_handler: {:?}", health_info);
    *visit_count += 1;

    // 返回
    HttpResponse::Ok().json(health_info)
}
