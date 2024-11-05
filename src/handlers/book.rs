use actix_web::{web, HttpResponse};

use crate::{dbaccess::book::get_books_db, error::MyError, state::AppState};

// get书函数
// 处理Get请求
pub async fn get_books(
    app_state: web::Data<AppState>,
    path: web::Path<(i32,)>, // 路径参数：/xxxx/{book_id}
) -> Result<HttpResponse, MyError> {
    let book_id = path.0; // 将路径参数转换为i32
    // 调用数据库访问函数，获取书
    get_books_db(&app_state.db, book_id)
    .await
    .map(|book| HttpResponse::Ok().json(book)) // 成功时返回200 OK和书信息
}