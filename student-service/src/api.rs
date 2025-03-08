use actix_web::{web, HttpResponse, Responder, get};
use crate::db;
use sqlx::{Pool, Postgres};
use log::error;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/students")
            .service(get_all_students)
            .service(get_student_by_id)
    );
}

#[get("")]
async fn get_all_students(db_pool: web::Data<Pool<Postgres>>) -> impl Responder {
    match db::get_all_students(&db_pool).await {
        Ok(students) => HttpResponse::Ok().json(students),
        Err(e) => {
            error!("Failed to fetch students: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[get("/{id}")]
async fn get_student_by_id(
    db_pool: web::Data<Pool<Postgres>>,
    id: web::Path<i32>,
) -> impl Responder {
    match db::get_student_by_id(&db_pool, id.into_inner()).await {
        Ok(Some(student)) => HttpResponse::Ok().json(student),
        Ok(None) => HttpResponse::NotFound().finish(),
        Err(e) => {
            error!("Failed to fetch student: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}