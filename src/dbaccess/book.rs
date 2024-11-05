use sqlx::PgPool;

use crate::{error::MyError, models::book::Book};



pub async fn get_books_db(
    pool: &PgPool,
    book_id: i32,
) -> Result<Book,MyError> {
     let row:Option<Book> = sqlx::query_as!(
        Book,
        r#"SELECT * FROM Books WHERE id = $1"#,
        book_id
    )
    .fetch_optional(pool)
    .await?;
    if let Some(book) = row{
        Ok(book)
    }else{
        Err(MyError::NotFound("Book ID not found".to_string()))
    }
}