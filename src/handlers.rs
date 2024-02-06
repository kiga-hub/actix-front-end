use actix_web::{web, Responder, HttpRequest, HttpResponse,http::header::ContentType,body::BoxBody,post};
use serde::Serialize;
use serde::Deserialize;
use actix_files as fs;

#[derive(Serialize)]
struct CustomType {
    name: &'static str,
}

pub async fn index(_req: HttpRequest) -> impl Responder {
    web::Bytes::from_static(b"Hello world!")
}

pub async fn index_custom_type() -> impl Responder {
    CustomType { name: "user" }
}

// Responder
impl Responder for CustomType {
    type Body = BoxBody;

    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        let body = serde_json::to_string(&self).unwrap();

        // Create response and set content type
        HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(body)
    }
}

// Define a type for the form data
#[derive(Deserialize)]
pub struct FormData {
    pub username: String,
    pub password: String,
}

// home page 
pub async fn home() -> impl Responder {
    fs::NamedFile::open("static/index.html").unwrap()
}

// Handler for POST requests
#[post("/home/login")]
pub async fn login(form: web::Form<FormData>) -> impl Responder {
    if form.username == "root" && form.password == "passwd" {
        let data = serde_json::json!({
            "message": "Logged in!",
            "image_url": "/static/src/ubuntu.jpg"
        });
        HttpResponse::Ok().body(data.to_string())
    } else {
        HttpResponse::BadRequest().body("Invalid email or password")
    }
}