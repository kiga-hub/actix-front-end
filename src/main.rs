use actix_files as fs;
use actix_web::{middleware, web, App, HttpResponse, HttpServer};
use std::{cell::Cell, sync::atomic::AtomicUsize, sync::Arc};
mod app_state;
mod handlers;
mod refresh_count;
mod routes;
mod stream;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let data = refresh_count::RefreshCount {
        local_count: Cell::new(0),
        global_count: Arc::new(AtomicUsize::new(0)),
    };

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(data.clone()))
            .service(
                web::scope("/api")
                    .configure(routes::scoped_config)
                    .route("/custom", web::get().to(handlers::index_custom_type))
                    .route("/index", web::get().to(handlers::index))
                    .service(stream::stream)
                    .service(refresh_count::show_count)
                    .service(refresh_count::add_one),
            )
            .service(handlers::login)
            .service(
                web::scope("/home")
                    .route("/login", web::get().to(handlers::home))
            )
            .service(
                web::scope("/form")
                    .configure(app_state::app_config)
            )
            .service(fs::Files::new("/", "/static"))
            .wrap(middleware::Logger::default())
            .route(
                "/",
                web::get().to(|| async { HttpResponse::Ok().body("/") }),
            )
    })
    .bind(("192.168.8.244", 9999))?
    .run()
    .await
}

#[cfg(test)]
mod tests {
    use actix_web::{
        body::to_bytes,
        dev::ServiceResponse,
        http::{
            header::{HeaderValue, CONTENT_TYPE},
            StatusCode,
        },
        test::{self, TestRequest},
        web::{Bytes, Form},
    };

    use super::*;

    trait BodyTest {
        fn as_str(&self) -> &str;
    }

    impl BodyTest for Bytes {
        fn as_str(&self) -> &str {
            std::str::from_utf8(self).unwrap()
        }
    }

    #[actix_web::test]
    async fn handle_post_1_unit_test() {
        let params = Form(app_state::MyParams {
            name: "John".to_owned(),
        });
        let resp = app_state::handle_post_1(params).await.unwrap();

        assert_eq!(resp.status(), StatusCode::OK);
        assert_eq!(
            resp.headers().get(CONTENT_TYPE).unwrap(),
            HeaderValue::from_static("text/plain")
        );

        let body = to_bytes(resp.into_body()).await.unwrap();
        assert_eq!(body.as_str(), "Your name is John");
    }

    #[actix_web::test]
    async fn handle_post_1_integration_test() {
        let app = test::init_service(App::new().configure(app_state::app_config)).await;
        let req = test::TestRequest::post()
            .uri("/post1")
            .set_form(app_state::MyParams {
                name: "John".to_owned(),
            })
            .to_request();
        let resp: ServiceResponse = test::call_service(&app, req).await;

        assert_eq!(resp.status(), StatusCode::OK);
        assert_eq!(
            resp.headers().get(CONTENT_TYPE).unwrap(),
            HeaderValue::from_static("text/plain")
        );
        let body = to_bytes(resp.into_body()).await.unwrap();
        assert_eq!(body.as_str(), "Your name is John");
    }

    #[actix_web::test]
    async fn handle_post_2_unit_test() {
        let state = TestRequest::default()
            .data(app_state::AppState {
                foo: "bar".to_owned(),
            })
            .to_http_request();
        let data = state
            .app_data::<actix_web::web::Data<app_state::AppState>>()
            .unwrap();
        let params = Form(app_state::MyParams {
            name: "John".to_owned(),
        });
        let resp = app_state::handle_post_2(data.clone(), params)
            .await
            .unwrap();

        assert_eq!(resp.status(), StatusCode::OK);
        assert_eq!(
            resp.headers().get(CONTENT_TYPE).unwrap(),
            HeaderValue::from_static("text/plain")
        );
        let body = to_bytes(resp.into_body()).await.unwrap();
        assert_eq!(
            body.as_str(),
            "Your name is John, and in AppState2 I have foo: bar"
        );
    }

    #[actix_web::test]
    async fn handle_post_2_integration_test() {
        let app = test::init_service(App::new().configure(app_state::app_config)).await;
        let req = test::TestRequest::post()
            .uri("/post2")
            .set_form(app_state::MyParams {
                name: "John".to_owned(),
            })
            .to_request();
        let resp: ServiceResponse = test::call_service(&app, req).await;

        assert_eq!(resp.status(), StatusCode::OK);
        assert_eq!(
            resp.headers().get(CONTENT_TYPE).unwrap(),
            HeaderValue::from_static("text/plain")
        );
        let resp = resp.into_parts().1;
        let body = to_bytes(resp.into_body()).await.unwrap();
        assert_eq!(
            body.as_str(),
            "Your name is John, and in AppState2 I have foo: bar"
        );
    }

    #[actix_web::test]
    async fn handle_post_3_unit_test() {
        let req = TestRequest::default().to_http_request();
        let params = Form(app_state::MyParams {
            name: "John".to_owned(),
        });
        let result = app_state::handle_post_3(req.clone(), params).await;
        use actix_web::Responder;
        let resp = result.respond_to(&req);
        assert_eq!(resp.status(), StatusCode::OK);
        assert_eq!(
            resp.headers().get(CONTENT_TYPE).unwrap(),
            HeaderValue::from_static("text/plain")
        );
        let body = match to_bytes(resp.into_body()).await {
            Ok(x) => x,
            _ => panic!(),
        };
        assert_eq!(body.as_str(), "Your name is John");
    }

    #[actix_web::test]
    async fn handle_post_3_integration_test() {
        let app = test::init_service(App::new().configure(app_state::app_config)).await;
        let req = test::TestRequest::post()
            .uri("/post3")
            .set_form(app_state::MyParams {
                name: "John".to_owned(),
            })
            .to_request();
        let resp: ServiceResponse = test::call_service(&app, req).await;

        assert_eq!(resp.status(), StatusCode::OK);
        assert_eq!(
            resp.headers().get(CONTENT_TYPE).unwrap(),
            HeaderValue::from_static("text/plain")
        );

        let body = to_bytes(resp.into_body()).await.unwrap();
        assert_eq!(body.as_str(), "Your name is John");
    }
}
