
use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct PredictionRequest {
    features: Vec<f32>,
}

#[derive(Serialize)]
struct PredictionResponse {
    prediction: f32,
}

// Placeholder for the actual model loading and prediction logic
fn predict(features: &[f32]) -> f32 {
    // In a real application, you would load a model and perform inference.
    // For this example, we just sum the features.
    features.iter().sum()
}

async fn handle_predict(req: web::Json<PredictionRequest>) -> impl Responder {
    if req.features.is_empty() {
        return HttpResponse::BadRequest().body("Features cannot be empty");
    }

    let prediction = predict(&req.features);
    let response = PredictionResponse { prediction };

    HttpResponse::Ok().json(response)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting server at http://127.0.0.1:8080");

    HttpServer::new(|| {
        App::new()
            .route("/predict", web::post().to(handle_predict))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test, App};

    #[actix_rt::test]
    async fn test_handle_predict() {
        let mut app = test::init_service(App::new().route("/predict", web::post().to(handle_predict))).await;
        let req_body = PredictionRequest { features: vec![1.0, 2.0, 3.0] };
        let req = test::TestRequest::post().uri("/predict").set_json(&req_body).to_request();
        let resp: PredictionResponse = test::read_body_json(test::call_service(&mut app, req).await).await;
        assert_eq!(resp.prediction, 6.0);
    }

    #[actix_rt::test]
    async fn test_handle_predict_empty_features() {
        let mut app = test::init_service(App::new().route("/predict", web::post().to(handle_predict))).await;
        let req_body = PredictionRequest { features: vec![] };
        let req = test::TestRequest::post().uri("/predict").set_json(&req_body).to_request();
        let resp = test::call_service(&mut app, req).await;
        assert_eq!(resp.status(), 400);
    }
}
