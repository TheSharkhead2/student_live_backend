// use actix_web::{delete, get, post, put, web, Error, HttpResponse, Responder};
// use diesel::prelude::*;

// use super::PostgressPool;

// use crate::models::{Class, ClassPayload, NewClass};
// use crate::PostgressPool;
// type DbError = Box<dyn std::error::Error + Send + Sync>;

// fn add_a_class(_code: String, _url: String, conn: &mut PgConnection) -> Result<Class, DbError> {
//     use crate::schema::classes::dsl::*;

//     let new_class = NewClass {
//         code: _code,
//         url: _url,
//         questions: Vec::new(),
//         upvotes: Vec::new(),
//     };

//     let res = diesel::insert_into(classes)
//         .values(&new_class)
//         .get_result(conn)?;

//     Ok(res)
// }

// #[post("/teacher/{class_id}/create")]
// pub fn create_class(
//     pool: web::Data<PostgressPool>,
//     mut payload: web::Json<ClassPayload>,
//     path: web::Path<(String,)>,
// ) -> Result<HttpResponse, actix_web::Error> {
//     let class = web::block(move || {
//         let conn = pool.get()?;
//     });

//     Ok(HttpResponse::Ok().body("hi"))
// }
use super::DbPool;
use actix_web::{delete, get, post, put, web, Error, HttpResponse, Responder};
use diesel::prelude::*;

use crate::models::{Class, ClassPayload, NewClass, QuestionPayload};

type DbError = Box<dyn std::error::Error + Send + Sync>;

#[get("/classes")]
async fn classes_index(pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {
    let classes = web::block(move || {
        let mut conn = pool.get()?;
        find_all(&mut conn)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(classes))
}

#[post("/classes")]
async fn classes_create(
    pool: web::Data<DbPool>,
    payload: web::Json<ClassPayload>,
) -> Result<HttpResponse, Error> {
    let class = web::block(move || {
        let mut conn = pool.get()?;
        add_a_class(payload.code.clone(), payload.url.clone(), &mut conn)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().body("Tweet#new"))
}

#[get("/classes/{id}")]
async fn classes_show(
    id: web::Path<String>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, Error> {
    let class = web::block(move || {
        let mut conn = pool.get()?;
        find_class_by_id(id.into_inner(), &mut conn)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(class))
}

#[put("/classes/{id}")]
async fn classes_update(
    id: web::Path<String>,
    payload: web::Json<ClassPayload>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, Error> {
    let class = web::block(move || {
        let mut conn = pool.get()?;
        update_url(id.into_inner(), payload.url.clone(), &mut conn)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(class))
}

#[put("/classes/{id}/question")]
async fn add_new_question(
    id: web::Path<String>,
    payload: web::Json<QuestionPayload>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, Error> {
    let class = web::block(move || {
        let mut conn = pool.get()?;
        add_question(id.into_inner(), payload.question.clone(), &mut conn)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(class))
}

fn add_a_class(_code: String, _url: String, conn: &mut PgConnection) -> Result<Class, DbError> {
    use crate::schema::classes::dsl::*;

    let new_class = NewClass {
        code: _code,
        url: _url,
        questions: Vec::new(),
        upvotes: Vec::new(),
    };

    let res = diesel::insert_into(classes)
        .values(&new_class)
        .get_result(conn)?;

    Ok(res)
}

fn find_all(conn: &mut PgConnection) -> Result<Vec<Class>, DbError> {
    use crate::schema::classes::dsl::*;

    let items = classes.load::<Class>(conn)?;

    Ok(items)
}

fn find_class_by_id(class_id: String, conn: &mut PgConnection) -> Result<Option<Class>, DbError> {
    use crate::schema::classes::dsl::*;

    let class = classes
        .filter(code.eq(class_id))
        .first::<Class>(conn)
        .optional()?;

    Ok(class)
}

fn update_url(
    class_code: String,
    _url: String,
    mut conn: &mut PgConnection,
) -> Result<Class, DbError> {
    use crate::schema::classes::dsl::*;

    let class_with_id = find_class_by_id(class_code, &mut conn)?.unwrap();

    let class = diesel::update(classes.find(class_with_id.id))
        .set(url.eq(_url))
        .get_result::<Class>(conn)?;

    Ok(class)
}

fn add_question(
    class_code: String,
    _question: String,
    mut conn: &mut PgConnection,
) -> Result<Class, DbError> {
    use crate::schema::classes::dsl::*;

    let class_with_id = find_class_by_id(class_code, &mut conn)?.unwrap();

    let mut class_questions = class_with_id.questions;
    let mut class_upvotes = class_with_id.upvotes;

    if class_questions.contains(&Some(_question.clone())) {
        let question_index = class_questions
            .iter()
            .position(|quest| quest == &Some(_question.clone()))
            .unwrap();
        let class_upvotes_voted: Vec<Option<i32>> = class_upvotes
            .iter()
            .enumerate()
            .map(|(index, vote_count)| {
                if index == question_index {
                    if let Some(vote_count) = vote_count {
                        Some(vote_count + 1)
                    } else {
                        Some(1)
                    }
                } else {
                    *vote_count
                }
            })
            .collect();

        let class = diesel::update(classes.find(class_with_id.id))
            .set(upvotes.eq(class_upvotes_voted))
            .get_result::<Class>(conn)?;

        Ok(class)
    } else {
        class_questions.push(Some(_question));
        class_upvotes.push(Some(1));

        let _class = diesel::update(classes.find(class_with_id.id))
            .set(questions.eq(class_questions))
            .get_result::<Class>(conn)?;

        let class = diesel::update(classes.find(class_with_id.id))
            .set(upvotes.eq(class_upvotes))
            .get_result::<Class>(conn)?;

        Ok(class)
    }
}
