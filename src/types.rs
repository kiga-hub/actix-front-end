use actix_web::{Responder, HttpRequest, HttpResponse,Responder};
use actix_http::body::BoxBody;

pub struct CustomType {
    name: &'static str,
}

pub impl Responder for CustomType {
    type Body = BoxBody;

    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        let body = serde_json::to_string(&self).unwrap();

        // Create response and set content type
        HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(body)
    }
}