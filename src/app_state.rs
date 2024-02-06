use actix_web::{web, HttpRequest, HttpResponse, Result};
use serde::{Deserialize, Serialize};

pub struct AppState {
    pub foo: String,
}

pub fn app_config(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("")
            .app_data(web::Data::new(AppState {
                foo: "bar".to_owned(),
            }))
            .service(web::resource("/info").route(web::get().to(index2)))
            .service(web::resource("/post1").route(web::post().to(handle_post_1)))
            .service(web::resource("/post2").route(web::post().to(handle_post_2)))
            .service(web::resource("/post3").route(web::post().to(handle_post_3))),
    );
}

pub async fn index2() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../static/form.html")))
}

#[derive(Serialize, Deserialize)]
pub struct MyParams {
    pub name: String,
}

/// Simple handle POST request
pub async fn handle_post_1(params: web::Form<MyParams>) -> Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .content_type("text/plain")
        .body(format!("Your name is {}", params.name)))
}

/// State and POST Params
pub async fn handle_post_2(
    state: web::Data<AppState>,
    params: web::Form<MyParams>,
) -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().content_type("text/plain").body(format!(
        "Your name is {}, and in AppState2 I have foo: {}",
        params.name, state.foo
    )))
}

/// Request and POST Params
pub async fn handle_post_3(req: HttpRequest, params: web::Form<MyParams>) -> Result<HttpResponse> {
    println!("Handling POST request: {req:?}");

    Ok(HttpResponse::Ok()
        .content_type("text/plain")
        .body(format!("Your name is {}", params.name)))
}
