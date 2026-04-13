use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use reqwest;
use chrono::Utc;
use actix_cors::Cors;
use std::env;


#[derive(Deserialize)]
struct NameQuery {
    name: String,
}

#[derive(Deserialize)]
struct GenderizeResponse {
    gender: Option<String>,
    probability: Option<f64>,
    count: Option<u32>,
}

#[derive(Serialize)]
struct ApiResponse {
    status: String,
    data: ResponseData,
}

#[derive(Serialize)]
struct ResponseData {
    name: String,
    gender: Option<String>,
    probability: Option<f64>,
    sample_size: u32,
    is_confident: bool,
    processed_at: String,
}

// Api call
// External API call
async fn get_gender(name: &str) -> Result<GenderizeResponse, reqwest::Error> {
    let client = reqwest::Client::new();

    let resp = client
        .get("https://api.genderize.io")
        .query(&[("name", name)])
        .send()
        .await?;

    let data = resp.json::<GenderizeResponse>().await?;
    Ok(data)
}

#[get("/api/classify")]
async fn classify(query: web::Query<NameQuery>) -> impl Responder {
    let name = query.name.trim();

    if name.is_empty() {
        return HttpResponse::BadRequest().json(
            serde_json::json!({
                "status": "error",
                "message": "Missing or empty name parameter"
            })
        );
    }

    let api_result = match get_gender(name).await {
        Ok(data) => data,
        Err(_) => {
            return HttpResponse::BadGateway().json(
                serde_json::json!({
                    "status": "error",
                    "message": "Upstream service failure"
                })
            );
        }
    };

    if api_result.gender.is_none() || api_result.count.unwrap_or(0) == 0 {
        return HttpResponse::UnprocessableEntity().json(
            serde_json::json!({
                "status": "error",
                "message": "No prediction available for the provided name"
            })
        );
    }

    let sample_size = api_result.count.unwrap_or(0);
    let probability = api_result.probability.unwrap_or(0.0);
    let is_confident = probability >= 0.7 && sample_size >= 100;

    let response = ApiResponse {
        status: "success".to_string(),
        data: ResponseData {
            name: name.to_string(),
            gender: api_result.gender,
            probability: api_result.probability,
            sample_size,
            is_confident,
            processed_at: Utc::now().to_rfc3339(),
        },
    };

    HttpResponse::Ok().json(response)
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let port: u16 = env::var("PORT")
    .unwrap_or_else(|_| "3000".to_string())
    .parse()
    .expect("PORT must be a number");

    HttpServer::new(|| {
        let cors = Cors::default()
        .allow_any_origin()
        .allowed_methods(vec!["GET"])
        .allow_any_header();
        App::new()
            .wrap(cors)
            .service(classify)
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}
