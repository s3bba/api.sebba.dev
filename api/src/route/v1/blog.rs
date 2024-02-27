use actix_web::{HttpResponse, Responder, web};
use chrono::Utc;
use sqlx::Error;
use sqlx::postgres::PgQueryResult;
use api_sebba_dev_base::model;
use crate::{PgDatabase, capture_error, util};
use crate::util::response::{HttpJsonErrorResponse, HttpJsonResponse};

pub fn configure(cfg: &mut web::ServiceConfig) {
    // region public routes
    cfg.route(
        "/blog/v1/posts",
        web::get().to(get_blog_posts),
    );

    cfg.route(
        "/blog/v1/posts/{id}",
        web::get().to(get_blog_post),
    );
    // endregion public routes

    // region authenticated routes
    cfg.route(
        "/blog/v1/admin/posts/hashes",
        web::get().to(get_blog_post_hashes),
    );

    cfg.route(
        "/blog/v1/admin/posts",
        web::post().to(create_blog_post)
    );

    cfg.route(
        "/blog/v1/admin/posts/{id}",
        web::patch().to(update_blog_post)
    );

    cfg.route(
        "/blog/v1/admin/posts/{id}",
        web::delete().to(delete_blog_post)
    );
    // endregion authenticated routes
}

/// Retrieves blog posts from the database.
///
/// Endpoint: `GET /blog/v1/posts`
async fn get_blog_posts(db: web::Data<PgDatabase>) -> impl Responder {
    // TODO (sebba): Implement pagination and filtering

    // language=postgresql
    const QUERY: &str = "select slug, title, description, tags, thumbnail_url, created_at from blog_posts order by created_at desc";
    let posts_result: Result<Vec<model::blog::BlogPostPreview>, Error> = sqlx::query_as(QUERY)
        .fetch_all(&db.pool)
        .await;

    match posts_result {
        Ok(posts) => {
            //tokio::time::sleep(tokio::time::Duration::from_secs(6)).await;

            let body: model::blog::BlogPostPreviewResponseBody = model::blog::BlogPostPreviewResponseBody {
                posts
            };
            HttpResponse::Ok().into_json(&body)
        }
        Err(err) => {
            capture_error!("Failed to fetch posts due to {:?} error", err);
            HttpResponse::InternalServerError().into_json_error("Failed to fetch posts")
        }
    }
}

/// Retrieves a blog post from the database.
///
/// Endpoint: `GET /blog/v1/posts/{id}`
async fn get_blog_post(
    path: web::Path<String>,
    db: web::Data<PgDatabase>
) -> impl Responder {
    let slug: String = path.into_inner();

    // language=postgresql
    const QUERY: &str = "select slug, title, tags, thumbnail_url, created_at, updated_at, content from blog_posts where slug = $1";
    let post_result: Result<model::blog::BlogPostResponseBody, Error> = sqlx::query_as(QUERY)
        .bind(&slug)
        .fetch_one(&db.pool)
        .await;

    match post_result {
        Ok(post) => {
            HttpResponse::Ok().into_json(&post)
        }
        Err(err) => {
            match err {
                Error::RowNotFound => {
                    HttpResponse::NotFound().into_json_error(format!("{} does not exist", slug))
                }
                _ => {
                    capture_error!("Failed to find {} due to {:?} error", slug, err);
                    HttpResponse::InternalServerError().into_json_error("Failed to find post")
                }
            }
        }
    }
}

/// Retrieves blog post hashes from the database.
///
/// Endpoint: `GET /blog/v1/admin/posts/hashes`
async fn get_blog_post_hashes(
    db: web::Data<PgDatabase>,
    _: util::authentication::Authenticated
) -> impl Responder {
    // language=postgresql
    const QUERY: &str = "select slug, hash from blog_posts";
    let result: Result<Vec<model::blog::BlogPostHash>, Error> = sqlx::query_as(QUERY)
        .fetch_all(&db.pool)
        .await;

    match result {
        Ok(posts) => {
            let model = model::blog::BlogPostHashesResponseBody {
                posts
            };

            HttpResponse::Ok().into_json(&model)
        }
        Err(err) => {
            capture_error!("Failed to fetch post hashes due to {:?} error", err);
            HttpResponse::InternalServerError().into_json_error("Failed to fetch post hashes")
        }
    }
}

/// Creates a blog post in the database.
///
/// Endpoint: `POST /blog/v1/admin/posts`
async fn create_blog_post(
    db: web::Data<PgDatabase>,
    post: web::Json<model::blog::BlogPostCreateRequestBody>,
    _: util::authentication::Authenticated
) -> impl Responder {
    let post: model::blog::BlogPostCreateRequestBody = post.into_inner();

    // TODO (sebba): Add validation

    // language=postgresql
    const QUERY: &str = "insert into blog_posts (slug, title, description, tags, thumbnail_url, created_at, content, hash) values ($1, $2, $3, $4, $5, $6, $7, $8)";
    let result: Result<PgQueryResult, Error> = sqlx::query(QUERY)
        .bind(post.slug)
        .bind(post.title)
        .bind(post.description)
        .bind(post.tags)
        .bind(post.thumbnail_url)
        .bind(Utc::now())
        .bind(post.content)
        .bind(post.hash)
        .execute(&db.pool)
        .await;

    match result {
        Ok(_) => {
            HttpResponse::Ok().finish()
        }
        Err(err) => {
            capture_error!("Failed to create post due to {:?} error", err);
            // TODO (sebba): Should client be able to see this error or is this generic one enough?
            HttpResponse::InternalServerError().into_json_error("Failed to create post")
        }
    }
}

/// Updates a blog post in the database.
///
/// Endpoint: `PATCH /blog/v1/admin/posts/{id}`
async fn update_blog_post(
    path: web::Path<String>,
    post: web::Json<model::blog::BlogPostUpdateRequestBody>,
    db: web::Data<PgDatabase>,
    _: util::authentication::Authenticated
) -> impl Responder {
    let slug: String = path.into_inner();
    let post: model::blog::BlogPostUpdateRequestBody = post.into_inner();

    // TODO (sebba): Add validation

    // language=postgresql
    const QUERY: &str = "update blog_posts set title = $1, description = $2, tags = $3,  thumbnail_url = $4, updated_at = $5, content = $6, hash = $7 where slug = $8";
    let result: Result<PgQueryResult, Error> = sqlx::query(QUERY)
        .bind(post.title)
        .bind(post.description)
        .bind(post.tags)
        .bind(post.thumbnail_url)
        .bind(Utc::now())
        .bind(post.content)
        .bind(post.hash)
        .bind(&slug)
        .execute(&db.pool)
        .await;

    match result {
        Ok(_) => {
            HttpResponse::Ok().finish()
        }
        Err(err) => {
            capture_error!("Failed to update post due to {:?} error", err);
            // TODO (sebba): Should client be able to see this error or is this generic one enough?
            HttpResponse::InternalServerError().into_json_error("Failed to update post")
        }
    }
}

/// Deletes a blog post from the database.
///
/// Endpoint: `DELETE /blog/v1/admin/posts/{id}`
async fn delete_blog_post(
    path: web::Path<String>,
    db: web::Data<PgDatabase>,
    _: util::authentication::Authenticated
) -> impl Responder {
    let slug: String = path.into_inner();

    // language=postgresql
    const QUERY: &str = "delete from blog_posts where slug = $1";
    let result: Result<PgQueryResult, Error> = sqlx::query(QUERY)
        .bind(&slug)
        .execute(&db.pool)
        .await;

    match result {
        Ok(_) => {
            HttpResponse::Ok().finish()
        }
        Err(err) => {
            capture_error!("Failed to delete post due to {:?} error", err);
            // TODO (sebba): Should client be able to see this error or is this generic one enough?
            HttpResponse::InternalServerError().into_json_error("Failed to delete post")
        }
    }
}
