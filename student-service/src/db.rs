use sqlx::{Pool, Postgres, postgres::PgPoolOptions, Error};
use crate::config::Config;
use crate::models::{Student, NewStudent};

pub async fn create_pool(config: &Config) -> Result<Pool<Postgres>, Error> {
    PgPoolOptions::new()
        .max_connections(5)
        .connect(&config.database_url)
        .await
}

pub async fn get_all_students(pool: &Pool<Postgres>) -> Result<Vec<Student>, Error> {
    sqlx::query_as::<_, Student>("SELECT * FROM students")
        .fetch_all(pool)
        .await
}

pub async fn get_student_by_id(pool: &Pool<Postgres>, id: i32) -> Result<Option<Student>, Error> {
    sqlx::query_as::<_, Student>("SELECT * FROM students WHERE id = $1")
        .bind(id)
        .fetch_optional(pool)
        .await
}

pub async fn create_student(pool: &Pool<Postgres>, student: &NewStudent) -> Result<(), Error> {
    sqlx::query(
        "INSERT INTO students (user_id, name, email, phone_number, student_id)
         VALUES ($1, $2, $3, $4, $5)"
    )
    .bind(student.user_id)
    .bind(&student.name)
    .bind(&student.email)
    .bind(&student.phone_number)
    .bind(&student.student_id)
    .execute(pool)
    .await?;
    
    Ok(())
}