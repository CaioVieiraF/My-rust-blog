use actix_web::{web, get};
use crate::AppState;
use crate::models::*;
use diesel::prelude::*;

#[get("/")]
async fn get_posts(data: web::Data<AppState>) -> web::Json<Vec<Post>> {
    use crate::schema::posts::dsl::*;

    let mut connection = data.database.get().unwrap();
    let all_posts = posts.filter(published.eq(0)).load::<Post>(&mut connection).expect("Error loading posts.");

    web::Json(all_posts)
}

#[get("/{id}")]
async fn get_post_by_id(data: web::Data<AppState>, arg: web::Path<i32>) -> web::Json<Post> {
    use crate::schema::posts::dsl::*;

    let mut connection = data.database.get().unwrap();
    let post_id = arg.into_inner();
    let all_posts = posts.filter(id.eq(post_id)).load::<Post>(&mut connection).expect("Error loading post");
    let query_post = &all_posts[0];

    web::Json(query_post.clone())
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("posts")
        .service(get_posts)
        .service(get_post_by_id)
    );
}
