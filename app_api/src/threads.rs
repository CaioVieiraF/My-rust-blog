use actix_web::{web, get, post};
use crate::{AppState, ResponseMessage, ResponseStatus};
use crate::models::*;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Serialize, Clone, Debug)]
struct NewPostRequest {
    title: String,
    body: String,
}

#[get("/")]
async fn get_posts(data: web::Data<AppState>) -> web::Json<Vec<Post>> {
    use crate::schema::posts::dsl::*;

    let mut connection = data.database.get().unwrap();
    let all_posts = posts.filter(published.eq(1)).load::<Post>(&mut connection).expect("Error loading posts.");

    web::Json(all_posts)
}

#[get("/{id}")]
async fn get_post_by_id(data: web::Data<AppState>, path: web::Path<String>) -> web::Json<Post> {
    use crate::schema::posts::dsl::*;

    let mut connection = data.database.get().unwrap();
    let post_id = path.into_inner();
    let all_posts = posts.filter(id.eq(post_id)).load::<Post>(&mut connection).expect("Error loading post");
    let query_post = &all_posts[0];

    web::Json(query_post.clone())
}

#[post("/new")]
async fn new_post(data: web::Data<AppState>, new: web::Json<NewPostRequest>) -> web::Json<ResponseMessage> {
    use crate::schema::posts;

    let new_id = Uuid::new_v4().to_string();
    let title = new.title.clone();
    let body = new.body.clone();
    let published = 0;
    let mut connection = data.database.get().unwrap();

    let the_new_post = NewPost {
        id: new_id,
        title,
        body,
        published,
    };

    let mut response_status: ResponseMessage = ResponseMessage::new(ResponseStatus::Succeed);

    diesel::insert_into(posts::table)
        .values(the_new_post)
        .execute(&mut connection)
        .unwrap_or_else(|_| {
            response_status = ResponseMessage::new(ResponseStatus::Failed);
            0
        });
    

    web::Json(response_status)
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("posts")
        .service(get_posts)
        .service(get_post_by_id)
        .service(new_post)
    );
}
