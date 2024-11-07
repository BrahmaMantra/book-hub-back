use actix_web::{web, HttpResponse};

use crate::{dbaccess::course::{get_courses_details_db, get_courses_for_teacher_db, post_new_course_db}, error::MyError, models::course::Course, state::AppState};

// 新课程处理函数
// 处理POST请求
pub async fn new_course(
    new_course: web::Json<Course>,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, MyError> {
    // 调用数据库访问函数，插入新课程
    post_new_course_db(&app_state.db, new_course.into()).await
    .map(|course| HttpResponse::Ok().json(course)) // 成功时返回200 OK和课程信息
}

// 获取教师的所有课程
// 处理GET请求
//#[get("/courses/{teacher_id}")]
pub async fn get_courses_for_teacher(
    app_state: web::Data<AppState>,
    params: web::Path<(usize,)>, // 路径参数：/xxxx/{teacher_id}
) -> Result<HttpResponse, MyError> {
    let teacher_id = i32::try_from(params.0).unwrap(); // 将路径参数转换为i32
    // 调用数据库访问函数，获取教师的所有课程
    get_courses_for_teacher_db(&app_state.db, teacher_id)
    .await
    .map(|courses| HttpResponse::Ok().json(courses)) // 成功时返回200 OK和课程列表
}

// 获取课程详情
// 处理GET请求
pub async fn get_course_detail(
    app_state: web::Data<AppState>,
    params: web::Path<(usize, usize)> // 路径参数：/xxxx/{teacher_id}/{course_id}
) -> Result<HttpResponse, MyError> {
    let (teacher_id, course_id) = (
        i32::try_from(params.0).unwrap(), 
        i32::try_from(params.1).unwrap()
    ); // 将路径参数转换为i32

    // 调用数据库访问函数，获取课程详情
    get_courses_details_db(&app_state.db, teacher_id, course_id)
    .await
    .map(|course| HttpResponse::Ok().json(course)) // 成功时返回200 OK和课程详情
}

#[cfg(test)]
mod tests {
    use std::sync::Mutex;
    use actix_web::{http::StatusCode, web};
    use dotenvy::dotenv;
    use sqlx::postgres::PgPoolOptions;
    use crate::{handlers::course::new_course, models::course::Course, state::AppState};
    use super::{get_course_detail, get_courses_for_teacher};

    // 测试新课程添加功能
    #[ignore]
    #[actix_rt::test]
    async fn post_course_test() {
        dotenv().ok(); // 加载环境变量
        let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set"); // 获取数据库URL
        let db_pool = PgPoolOptions::new().connect(&db_url).await.expect("Failed to create pool"); // 创建数据库连接池
        let app_state = web::Data::new(AppState {
            health_check_response: "I'm OK".to_string(),
            visit_count: Mutex::new(0),
            db: db_pool,
        });
        let course = web::Json(Course {
            teacher_id: Some(1),
            id: 1,
            name: Some("Math".to_string()),
            time: None,
        });

        // 调用新课程处理函数
        let resp = new_course(course, app_state).await;

        // 断言响应状态码为200 OK
        assert_eq!(resp.unwrap().status(), StatusCode::OK);
    }

    // 测试获取教师的所有课程功能
    #[actix_rt::test]
    async fn get_all_courses_success() {
        dotenv().ok(); // 加载环境变量
        let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set"); // 获取数据库URL
        let db_pool = PgPoolOptions::new().connect(&db_url).await.expect("Failed to create pool"); // 创建数据库连接池
        let app_state = web::Data::new(AppState {
            health_check_response: "I'm OK".to_string(),
            visit_count: Mutex::new(0),
            db: db_pool,
        });

        let teacher_id: web::Path<(usize,)> = web::Path::from((1,)); // 设置教师ID
        let resp = get_courses_for_teacher(app_state, teacher_id).await.unwrap(); // 调用获取课程函数
        assert_eq!(resp.status(), StatusCode::OK); // 断言响应状态码为200 OK
    }

    // 测试获取课程详情功能
    #[actix_rt::test]
    async fn get_one_course_success() {
        dotenv().ok(); // 加载环境变量
        let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set"); // 获取数据库URL
        let db_pool = PgPoolOptions::new().connect(&db_url).await.expect("Failed to create pool"); // 创建数据库连接池
        let app_state = web::Data::new(AppState {
            health_check_response: "I'm OK".to_string(),
            visit_count: Mutex::new(0),
            db: db_pool,
        });

        let params: web::Path<(usize, usize)> = web::Path::from((1, 1)); // 设置教师ID和课程ID
        let resp = get_course_detail(app_state, params).await; // 调用获取课程详情函数
        assert_eq!(resp.unwrap().status(), StatusCode::OK); // 断言响应状态码为200 OK
    }
}