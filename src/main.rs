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

// ============================================================================
// Data Structures for Mastercard ISO 8583
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthorizationRequest {
    pub mti: String,                    // Message Type Indicator (0100)
    pub de2: String,                    // Primary Account Number (PAN)
    pub de3: String,                    // Processing Code
    pub de4: String,                    // Amount
    pub de7: String,                    // Transmission Date & Time (MMDDhhmmss UTC)
    pub de11: String,                   // Systems Trace Audit Number (STAN)
    pub de18: String,                   // Merchant Type
    pub de32: String,                   // Acquiring Institution ID
    pub de48: String,                   // Additional Data (Private Use)
    pub de49: String,                   // Currency Code
    pub de61: String,                   // POS Data
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthorizationResponse {
    pub mti: String,                    // Message Type Indicator (0110)
    pub de2: String,                    // Echo: Primary Account Number
    pub de3: String,                    // Echo: Processing Code
    pub de4: String,                    // Echo: Amount
    pub de7: String,                    // Echo: Transmission Date & Time
    pub de11: String,                   // Echo: STAN
    pub de18: String,                   // Echo: Merchant Type
    pub de32: String,                   // Echo: Acquiring Institution ID
    pub de39: String,                   // Response Code (00=success, 05=invalid, 51=insufficient)
    pub de48: String,                   // Echo: Additional Data
    pub de49: String,                   // Echo: Currency Code
    pub de61: String,                   // Echo: POS Data
    pub response_message: String,       // Human-readable response
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReversalRequest {
    pub mti: String,                    // Message Type Indicator (0400)
    pub de2: String,                    // Primary Account Number
    pub de3: String,                    // Processing Code
    pub de4: String,                    // Amount
    pub de7: String,                    // Transmission Date & Time
    pub de11: String,                   // Systems Trace Audit Number (STAN)
    pub de18: String,                   // Merchant Type
    pub de22: String,                   // Point of Service Entry Mode
    pub de32: String,                   // Acquiring Institution ID
    pub de39: String,                   // Original Response Code
    pub de48: String,                   // Additional Data
    pub de49: String,                   // Currency Code
    pub de61: String,                   // POS Data
    pub de90: String,                   // Original Data Elements
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReversalResponse {
    pub mti: String,                    // Message Type Indicator (0410)
    pub de2: String,                    // Echo: Primary Account Number
    pub de3: String,                    // Echo: Processing Code
    pub de4: String,                    // Echo: Amount
    pub de7: String,                    // Echo: Transmission Date & Time
    pub de11: String,                   // Echo: STAN
    pub de18: String,                   // Echo: Merchant Type
    pub de32: String,                   // Echo: Acquiring Institution ID
    pub de39: String,                   // Response Code (00=success, 94=duplicate)
    pub de48: String,                   // Echo: Additional Data
    pub de49: String,                   // Echo: Currency Code
    pub de61: String,                   // Echo: POS Data
    pub de90: String,                   // Echo: Original Data Elements
    pub response_message: String,       // Human-readable response
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
    // HashMap: STAN -> Transaction (simulate database of authorized transactions)
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

    // Validation: Check MTI
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
            de39: "03".to_string(), // Invalid MTI
            de48: payload.de48.clone(),
            de49: payload.de49.clone(),
            de61: payload.de61.clone(),
            response_message: "Invalid MTI for Authorization Request".to_string(),
        };
        println!("\n========== AUTHORIZATION RESPONSE ==========");
        println!("{}", serde_json::to_string_pretty(&response).unwrap());
        return (StatusCode::OK, Json(response));
    }

    // Business Logic: Approve if PAN starts with "4", else reject
    let response_code = if payload.de2.starts_with('4') {
        "00" // Success
    } else {
        "05" // Not authorized by financial institution
    };

    // Store authorized transaction in memory
    if response_code == "00" {
        let transaction = Transaction {
            pan: payload.de2.clone(),
            amount: payload.de4.clone(),
            stan: payload.de11.clone(),
            timestamp: payload.de7.clone(),
            response_code: response_code.to_string(),
        };
        state.authorized_transactions.lock().unwrap().insert(
            payload.de11.clone(),
            transaction,
        );
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
        response_message: match response_code {
            "00" => "Transaction Approved".to_string(),
            "05" => "Transaction Not Authorized".to_string(),
            _ => "Unknown Response".to_string(),
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

    // Validation: Check MTI
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
            de39: "03".to_string(), // Invalid MTI
            de48: payload.de48.clone(),
            de49: payload.de49.clone(),
            de61: payload.de61.clone(),
            de90: payload.de90.clone(),
            response_message: "Invalid MTI for Reversal Request".to_string(),
        };
        println!("\n========== REVERSAL RESPONSE ==========");
        println!("{}", serde_json::to_string_pretty(&response).unwrap());
        return (StatusCode::OK, Json(response));
    }

    // Business Logic: Check if original transaction exists
    let transactions = state.authorized_transactions.lock().unwrap();
    let response_code = if transactions.contains_key(&payload.de11) {
        "00" // Success - transaction found and reversed
    } else {
        "94" // Duplicate reversal or reversal amount mismatch
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
        response_message: match response_code {
            "00" => "Reversal Approved".to_string(),
            "94" => "Duplicate Reversal or Original Not Found".to_string(),
            _ => "Unknown Response".to_string(),
        },
    };

    println!("\n========== REVERSAL RESPONSE ==========");
    println!("{}", serde_json::to_string_pretty(&response).unwrap());

    (StatusCode::OK, Json(response))
}

// ============================================================================
// Main Application
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

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .expect("Failed to bind to port 3000");

    println!("\n╔════════════════════════════════════════════════════════════════╗");
    println!("║         Mastercard ISO 8583 Mock API Server                   ║");
    println!("║                  Server running on port 3000                   ║");
    println!("║                                                                ║");
    println!("║  Endpoints:                                                    ║");
    println!("║    POST /authorize  - Authorization request (MTI 0100)        ║");
    println!("║    POST /reversal   - Reversal request (MTI 0400)             ║");
    println!("╚════════════════════════════════════════════════════════════════╝\n");

    axum::serve(listener, app)
        .await
        .expect("Server error");
}
