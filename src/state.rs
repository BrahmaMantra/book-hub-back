use std::sync::Mutex;
use sqlx::PgPool;


pub struct AppState{
    pub health_check_response:String,
    pub visit_count: Mutex<u32>,
    //pub courses:Mutex<Vec<Course>>//在内存里存储
    pub db: PgPool //数据库连接池
}