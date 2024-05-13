mod database;
mod models;
mod schema;

use actix_web::{web, App, HttpResponse, HttpServer, Responder, Result};
use database::*;
use diesel::prelude::*;
use models::*;
use serde::Deserialize;

async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

async fn list_organizations(pool: web::Data<DbPool>) -> Result<impl Responder> {
    use self::schema::organizations::dsl::*;

    let results = web::block(move || {
        let mut connection = pool.get().expect("couldn't get db connection from pool");

        let results: Vec<Organization> = organizations
            .limit(10)
            .select(Organization::as_select())
            .load(&mut connection)
            .expect("Error loading organizations");

        results
    })
    .await
    .unwrap();

    Ok(web::Json(results))
}

#[derive(Deserialize)]
struct OrganizationRequest {
    name: String,
}

async fn create_organization(
    pool: web::Data<DbPool>,
    organization_request: web::Json<OrganizationRequest>,
) -> Result<impl Responder> {
    use self::schema::organizations::dsl::*;

    web::block(move || {
        let new_organization = NewOrganization {
            name: organization_request.name.as_str(),
        };
        // Obtaining a connection from the pool is also a potentially blocking operation.
        // So, it should be called within the `web::block` closure, as well.
        let mut connection = pool.get().expect("couldn't get db connection from pool");

        diesel::insert_into(organizations)
            .values(&new_organization)
            .returning(Organization::as_returning())
            .get_result(&mut connection)
            .expect("Error saving new organization");
    })
    .await
    .unwrap();

    Ok("ok")
}

async fn list_posts(pool: web::Data<DbPool>) -> Result<impl Responder> {
    use self::schema::posts::dsl::*;

    let results = web::block(move || {
        // Obtaining a connection from the pool is also a potentially blocking operation.
        // So, it should be called within the `web::block` closure, as well.
        let mut connection = pool.get().expect("couldn't get db connection from pool");

        let results: Vec<Post> = posts
            // .filter(published.eq(true))
            .limit(10)
            .select(Post::as_select())
            .load(&mut connection)
            .expect("Error loading posts");

        results
    })
    .await
    .unwrap();

    Ok(web::Json(results))
}

#[derive(Deserialize)]
struct PostRequest {
    title: String,
    body: String,
}

async fn create_post(
    pool: web::Data<DbPool>,
    post_request: web::Json<PostRequest>,
) -> Result<impl Responder> {
    use self::schema::posts::dsl::*;

    web::block(move || {
        let new_post = NewPost {
            title: post_request.title.as_str(),
            body: post_request.body.as_str(),
        };
        // Obtaining a connection from the pool is also a potentially blocking operation.
        // So, it should be called within the `web::block` closure, as well.
        let mut connection = pool.get().expect("couldn't get db connection from pool");

        diesel::insert_into(posts)
            .values(&new_post)
            .returning(Post::as_returning())
            .get_result(&mut connection)
            .expect("Error saving new post");
    })
    .await
    .unwrap();

    Ok("ok")
}

async fn list_users(pool: web::Data<DbPool>) -> Result<impl Responder> {
    use self::schema::users::dsl::*;

    let results = web::block(move || {
        // Obtaining a connection from the pool is also a potentially blocking operation.
        // So, it should be called within the `web::block` closure, as well.
        let mut connection = pool.get().expect("couldn't get db connection from pool");

        let results: Vec<User> = users
            .limit(10)
            .select(User::as_select())
            .load(&mut connection)
            .expect("Error loading posts");

        results
    })
    .await
    .unwrap();

    Ok(web::Json(results))
}

#[derive(Deserialize)]
struct UserRequest {
    name: String,
}

async fn create_user(
    pool: web::Data<DbPool>,
    user_request: web::Json<UserRequest>,
) -> Result<impl Responder> {
    use self::schema::users::dsl::*;

    web::block(move || {
        let new_user = NewUser {
            name: user_request.name.as_str(),
        };
        // Obtaining a connection from the pool is also a potentially blocking operation.
        // So, it should be called within the `web::block` closure, as well.
        let mut connection = pool.get().expect("couldn't get db connection from pool");

        diesel::insert_into(users)
            .values(&new_user)
            .returning(User::as_returning())
            .get_result(&mut connection)
            .expect("Error saving new user");
    })
    .await
    .unwrap();

    Ok("ok")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = get_connection_pool();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .route("/", web::get().to(hello))
            .route("/organizations", web::get().to(list_organizations))
            .route("/organizations", web::post().to(create_organization))
            .route("/posts", web::get().to(list_posts))
            .route("/posts", web::post().to(create_post))
            .route("/users", web::get().to(list_users))
            .route("/users", web::post().to(create_user))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
