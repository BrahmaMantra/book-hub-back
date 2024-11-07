use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::web::ServiceConfig;
use actix_web::{http, web, App, HttpServer, Route};
use dotenvy::dotenv;
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use state::AppState;
use std::{env, io};
use std::sync::Mutex;

mod handlers;

mod routers;

mod state;

mod models;

mod dbaccess;

mod error;

use routers::*;

#[actix_web::main]
async fn main() -> io::Result<()> {
    // 加载 .env 文件中的环境变量
    dotenv().ok();

    // 获取数据库URL
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set");

    // 创建数据库连接池
    let db_pool = PgPoolOptions::new()
        .connect(&database_url)
        .await
        .expect("Failed to create pool");

    // 创建共享状态数据
    let shared_data = web::Data::new(AppState {
        health_check_response: "I'm OK".to_string(),
        visit_count: Mutex::new(0),
        // courses: Mutex::new(vec![]), // 如果需要，可以取消注释
        db: db_pool,
    });
    
    // 配置Actix Web应用程序https://your-backend-url.com
    let app = move || {
        let cors = Cors::default() // i wanna configure cors in cfg below
            .allow_any_origin()
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            .allowed_header(http::header::CONTENT_TYPE)
            .max_age(3600);
        App::new()
            .wrap(cors)
            .app_data(shared_data.clone()) // 共享状态数据
            .configure(general_routes) // 配置通用路由
            .configure(course_routes) // 配置课程路由
            .configure(book_routes) // 配置书路由
    };

    // 启动HTTP服务器并绑定到指定地址和端口
    HttpServer::new(app).bind("127.0.0.1:8000")?.run().await
}
