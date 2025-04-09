use actix_web::{web, App, HttpServer, HttpResponse, Responder, get};
use dashmap::DashMap;
use serde::{Serialize, Deserialize};
use std::sync::Arc;
use std::time::{Duration, Instant};
use std::thread;
use rand::Rng;

#[derive(Deserialize)]
struct QueryParams {
    id: Option<u64>,
    query: Option<String>,
    delay: Option<u64>,
}

#[derive(Serialize, Clone)]
struct ResponseData {
    id: u64,
    query: String,
    result: String,
    timestamp: u64,
    processing_time_ms: u64,
}

type Cache = Arc<DashMap<String, ResponseData>>;

fn process_data(id: u64, query: &str, delay: u64) -> ResponseData {
    let start = Instant::now();
    
    if delay > 0 {
        thread::sleep(Duration::from_millis(delay));
    }
    
    let result = format!("Processed data for id={} with query='{}'", id, query);

    ResponseData {
        id,
        query: query.to_string(),
        result,
        timestamp: chrono::Utc::now().timestamp() as u64,
        processing_time_ms: start.elapsed().as_millis() as u64,
    }
}

#[get("/api/data")]
async fn get_data(
    query: web::Query<QueryParams>,
    cache: web::Data<Cache>,
    threadpool: web::Data<threadpool::ThreadPool>,
) -> impl Responder {
    let id = query.id.unwrap_or(0);
    let query_text = query.query.clone().unwrap_or_else(|| "default".to_string());
    let delay = query.delay.unwrap_or(0).min(5000);
    
    let cache_key = format!("{}:{}", id, query_text);
    
    if let Some(cached_data) = cache.get(&cache_key) {
        return HttpResponse::Ok().json(cached_data.value());
    }
    
    let cache_ref = cache.clone();
    let query_text_clone = query_text.clone();

    let (tx, rx) = tokio::sync::oneshot::channel();
    
    threadpool.execute(move || {
        let result = process_data(id, &query_text_clone, delay);
        cache_ref.insert(cache_key, result.clone());
        let _ = tx.send(result);
    });
    
    match rx.await {
        Ok(data) => HttpResponse::Ok().json(data),
        Err(_) => HttpResponse::InternalServerError().json("Помилка обробки запиту"),
    }
}

#[get("/health")]
async fn health_check() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({
        "status": "ok",
        "timestamp": chrono::Utc::now().timestamp()
    }))
}

#[get("/api/clear-cache")]
async fn clear_cache(cache: web::Data<Cache>) -> impl Responder {
    let count = cache.len();
    cache.clear();
    HttpResponse::Ok().json(serde_json::json!({
        "status": "ok",
        "cleared_entries": count
    }))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));
    
    let cache = Arc::new(DashMap::<String, ResponseData>::new());
    
    let num_threads = num_cpus::get();
    let threadpool = threadpool::ThreadPool::new(num_threads);

    println!("Starting server with {} worker threads", num_threads);
    
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(cache.clone()))
            .app_data(web::Data::new(threadpool.clone()))
            .service(get_data)
            .service(health_check)
            .service(clear_cache)
    })
        .workers(num_threads)
        .bind("127.0.0.1:8080")?
        .run()
        .await
}