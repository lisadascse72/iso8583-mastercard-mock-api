use axum::{
    extract::{State, Json},
    http::StatusCode,
    response::IntoResponse,
    routing::post,
    Router,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tower_http::cors::CorsLayer;
use std::env;

// ============================================================================
// Data Structures for Mastercard ISO 8583
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthorizationRequest {
    pub mti: String,
    pub de2: String,
    pub de3: String,
    pub de4: String,
    pub de7: String,
    pub de11: String,
    pub de18: String,
    pub de32: String,
    pub de48: String,
    pub de49: String,
    pub de61: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthorizationResponse {
    pub mti: String,
    pub de2: String,
    pub de3: String,
    pub de4: String,
    pub de7: String,
    pub de11: String,
    pub de18: String,
    pub de32: String,
    pub de39: String,
    pub de48: String,
    pub de49: String,
    pub de61: String,
    pub response_message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReversalRequest {
    pub mti: String,
    pub de2: String,
    pub de3: String,
    pub de4: String,
    pub de7: String,
    pub de11: String,
    pub de18: String,
    pub de22: String,
    pub de32: String,
    pub de39: String,
    pub de48: String,
    pub de49: String,
    pub de61: String,
    pub de90: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReversalResponse {
    pub mti: String,
    pub de2: String,
    pub de3: String,
    pub de4: String,
    pub de7: String,
    pub de11: String,
    pub de18: String,
    pub de32: String,
    pub de39: String,
    pub de48: String,
    pub de49: String,
    pub de61: String,
    pub de90: String,
    pub response_message: String,
}

// ============================================================================
// Transaction Storage
// ============================================================================

#[derive(Debug, Clone)]
pub struct Transaction {
    pub pan: String,
    pub amount: String,
    pub stan: String,
    pub timestamp: String,
    pub response_code: String,
}

pub struct AppState {
    pub authorized_transactions: Mutex<HashMap<String, Transaction>>,
}

// ============================================================================
// Request Handlers
// ============================================================================

async fn authorize(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<AuthorizationRequest>,
) -> impl IntoResponse {

    println!("\n========== AUTHORIZATION REQUEST ==========");
    println!("{}", serde_json::to_string_pretty(&payload).unwrap());

    if payload.mti != "0100" {
        let response = AuthorizationResponse {
            mti: "0110".to_string(),
            de2: payload.de2.clone(),
            de3: payload.de3.clone(),
            de4: payload.de4.clone(),
            de7: payload.de7.clone(),
            de11: payload.de11.clone(),
            de18: payload.de18.clone(),
            de32: payload.de32.clone(),
            de39: "03".to_string(),
            de48: payload.de48.clone(),
            de49: payload.de49.clone(),
            de61: payload.de61.clone(),
            response_message: "Invalid MTI for Authorization Request".to_string(),
        };

        return (StatusCode::OK, Json(response));
    }

    let response_code = if payload.de2.starts_with('4') {
        "00"
    } else {
        "05"
    };

    if response_code == "00" {
        let transaction = Transaction {
            pan: payload.de2.clone(),
            amount: payload.de4.clone(),
            stan: payload.de11.clone(),
            timestamp: payload.de7.clone(),
            response_code: response_code.to_string(),
        };

        state
            .authorized_transactions
            .lock()
            .unwrap()
            .insert(payload.de11.clone(), transaction);
    }

    let response = AuthorizationResponse {
        mti: "0110".to_string(),
        de2: payload.de2.clone(),
        de3: payload.de3.clone(),
        de4: payload.de4.clone(),
        de7: payload.de7.clone(),
        de11: payload.de11.clone(),
        de18: payload.de18.clone(),
        de32: payload.de32.clone(),
        de39: response_code.to_string(),
        de48: payload.de48.clone(),
        de49: payload.de49.clone(),
        de61: payload.de61.clone(),
        response_message: if response_code == "00" {
            "Transaction Approved".to_string()
        } else {
            "Transaction Not Authorized".to_string()
        },
    };

    println!("\n========== AUTHORIZATION RESPONSE ==========");
    println!("{}", serde_json::to_string_pretty(&response).unwrap());

    (StatusCode::OK, Json(response))
}

async fn reversal(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<ReversalRequest>,
) -> impl IntoResponse {

    println!("\n========== REVERSAL REQUEST ==========");
    println!("{}", serde_json::to_string_pretty(&payload).unwrap());

    if payload.mti != "0400" {
        let response = ReversalResponse {
            mti: "0410".to_string(),
            de2: payload.de2.clone(),
            de3: payload.de3.clone(),
            de4: payload.de4.clone(),
            de7: payload.de7.clone(),
            de11: payload.de11.clone(),
            de18: payload.de18.clone(),
            de32: payload.de32.clone(),
            de39: "03".to_string(),
            de48: payload.de48.clone(),
            de49: payload.de49.clone(),
            de61: payload.de61.clone(),
            de90: payload.de90.clone(),
            response_message: "Invalid MTI for Reversal Request".to_string(),
        };

        return (StatusCode::OK, Json(response));
    }

    let transactions = state.authorized_transactions.lock().unwrap();
    let response_code = if transactions.contains_key(&payload.de11) {
        "00"
    } else {
        "94"
    };

    let response = ReversalResponse {
        mti: "0410".to_string(),
        de2: payload.de2.clone(),
        de3: payload.de3.clone(),
        de4: payload.de4.clone(),
        de7: payload.de7.clone(),
        de11: payload.de11.clone(),
        de18: payload.de18.clone(),
        de32: payload.de32.clone(),
        de39: response_code.to_string(),
        de48: payload.de48.clone(),
        de49: payload.de49.clone(),
        de61: payload.de61.clone(),
        de90: payload.de90.clone(),
        response_message: if response_code == "00" {
            "Reversal Approved".to_string()
        } else {
            "Duplicate Reversal or Original Not Found".to_string()
        },
    };

    println!("\n========== REVERSAL RESPONSE ==========");
    println!("{}", serde_json::to_string_pretty(&response).unwrap());

    (StatusCode::OK, Json(response))
}

// ============================================================================
// Main Application (Render-Compatible)
// ============================================================================

#[tokio::main]
async fn main() {
    let state = Arc::new(AppState {
        authorized_transactions: Mutex::new(HashMap::new()),
    });

    let app = Router::new()
        .route("/authorize", post(authorize))
        .route("/reversal", post(reversal))
        .layer(CorsLayer::permissive())
        .with_state(state);

    // ✅ REQUIRED FOR RENDER
    let port = env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let bind_addr = format!("0.0.0.0:{}", port);

    let listener = tokio::net::TcpListener::bind(&bind_addr)
        .await
        .expect("Failed to bind to port");

    println!("\n╔════════════════════════════════════════════════════════════════╗");
    println!("║   Mastercard ISO 8583 Mock API Server (Rust + Axum)           ║");
    println!("║   Server running on {}", bind_addr);
    println!("║                                                                ║");
    println!("║   POST /authorize  → MTI 0100                                 ║");
    println!("║   POST /reversal   → MTI 0400                                 ║");
    println!("╚════════════════════════════════════════════════════════════════╝\n");

    axum::serve(listener, app)
        .await
        .expect("Server error");
}
