use sqlx::PgPool;

use crate::{error::MyError, models::course::Course};



pub async fn get_courses_for_teacher_db(
    pool: &PgPool,
    teacher_id: i32,
) -> Result<Vec<Course>,MyError> {
     let rows:Vec<Course> = sqlx::query_as!(
        Course,
        r#"SELECT * FROM courses WHERE teacher_id = $1"#,
        teacher_id
    )
    .fetch_all(pool)
    .await?;
    Ok(rows)
}

pub async fn get_courses_details_db(pool: &PgPool, teacher_id: i32, course_id: i32) -> Result<Course,MyError>{
    let rows = sqlx::query_as!(
        Course,
        r#"
        SELECT * 
        FROM courses 
        WHERE teacher_id = $1 and id = $2"#,
        teacher_id,
        course_id
    ).fetch_optional(pool)
    .await?;
    if let Some(course) = rows{
        Ok(course)
    }else {
        Err(MyError::NotFound("Course ID not found".to_string()))
    }
}

pub async fn post_new_course_db(pool: &PgPool, new_course: Course) -> Result<Course,MyError>{
    let row = sqlx::query_as!(
        Course,
        r#"
        INSERT INTO courses (id, teacher_id, name)
        VALUES ($1, $2, $3)
        RETURNING id, teacher_id, name,time
        "#,
        new_course.id,
        new_course.teacher_id,
        new_course.name
    )
    .fetch_one(pool)
    .await?;
    Ok(row)
}
#[allow(unused)]
pub async fn delete_course_db(pool:&PgPool, teacher_id:i32, course_id:i32) -> Result<String,MyError>{
    let _row = sqlx::query!(
        r#"
        DELETE FROM courses
        WHERE teacher_id = $1 and id = $2
        "#,
        teacher_id,
        course_id
    )
    .execute(pool)
    .await?;
    Ok(format!("Course with id {} deleted",course_id))
}