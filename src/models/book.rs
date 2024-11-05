use actix_web::web;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

// 定义 Book 结构体，表示书籍信息
#[derive(Debug, Serialize, Deserialize, Clone, sqlx::FromRow)]
pub struct Book {
    // 书籍 ID，自增主键
    pub id: i32,
    // 书籍标题
    pub title: String,
    // 书籍作者
    pub author: Option<String>,
    // 书籍描述
    pub description: Option<String>,
    // 书籍状态
    pub status: Option<i32>,
    // 书籍评分
    pub rating: Option<f32>,
    // 添加日期
    pub addedDate: Option<NaiveDateTime>,
    // 封面 URL
    pub cover_url: Option<String>,
}

// 实现 From trait，用于将 web::Json<Book> 转换为 Book 结构体
impl From<web::Json<Book>> for Book {
    fn from(book: web::Json<Book>) -> Self {
        Book {
            id: book.id,
            title: book.title.clone(),
            author: book.author.clone(),
            description: book.description.clone(),
            status: book.status,
            rating: book.rating,
            addedDate: book.addedDate,
            cover_url: book.cover_url.clone(),
        }
    }
}