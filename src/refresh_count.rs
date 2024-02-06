// File: refresh_count.rs

use actix_web::{web, Responder,get};
use std::sync::Arc;
use std::cell::Cell;
use std::sync::atomic::{AtomicUsize, Ordering};

#[derive(Clone)]
pub struct RefreshCount {
    pub local_count: Cell<usize>,
    pub global_count: Arc<AtomicUsize>,
}

#[get("/get")]
pub async fn show_count(data: web::Data<RefreshCount>) -> impl Responder {
    format!(
        "global_count: {}\nlocal_count: {}",
        data.global_count.load(Ordering::Relaxed),
        data.local_count.get()
    )
}

#[get("/add")]
pub async fn add_one(data: web::Data<RefreshCount>) -> impl Responder {
    data.global_count.fetch_add(1, Ordering::Relaxed);

    let local_count = data.local_count.get();
    data.local_count.set(local_count + 1);

    format!(
        "global_count: {}\nlocal_count: {}",
        data.global_count.load(Ordering::Relaxed),
        data.local_count.get()
    )
}