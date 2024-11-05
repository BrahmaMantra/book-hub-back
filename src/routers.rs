use super::handlers::*;
use actix_web::web;
use book::get_books;
use course::{get_course_detail, get_courses_for_teacher, new_course};
use general::health_check_handler;

pub fn general_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/health", web::get().to(health_check_handler));
}

pub fn course_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/courses")
            .route("/", web::post().to(new_course)) //curl -X POST localhost:8000/courses/ -H "Content-Type: application/json" -d '{"teacher_id":1,"id":7,"name":"First course"}'
            .route("{teacher_id}", web::get().to(get_courses_for_teacher)) //curl localhost:8000/courses/1
            .route(
                "/{teacher_id}/{course_id}",
                web::get().to(get_course_detail),
            ), //curl localhost:8000/courses/1/1
    );
}

pub fn book_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/library")
        .route("/{book_id}", web::get().to(get_books)));////curl localhost:8000/library/1
}
