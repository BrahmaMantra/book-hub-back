use actix_web::web;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
// use crate::models::course::Course;
#[derive(Debug, Serialize, Deserialize,Clone,sqlx::FromRow)]
pub struct Course{
    pub id :i32,
    pub teacher_id:Option<i32>,
    pub name:Option<String>,
    pub time:Option<NaiveDateTime>,
}
impl From<web::Json<Course>>for Course{
    fn from(course:web::Json<Course>)->Self{
        Course{
            teacher_id:course.teacher_id,
            id:course.id,
            name:course.name.clone(),
            time:course.time,
        }
    }
}