# ISO 8583 Mastercard Mock API (Rust)

## 📌 Project Overview

This project is a **mock payment processing backend** that simulates **Mastercard ISO 8583–based authorization and reversal flows** using **Rust** and the **Axum** web framework.

It is designed purely for **learning, demonstration, and interview preparation**, helping developers understand how card-based financial transactions are processed at the backend—without connecting to any real bank, NPCI, or Mastercard network.

The system exposes RESTful APIs that accept **JSON-formatted ISO 8583–like messages**, where each field represents a specific **ISO 8583 Data Element (DE)**.

---

## 🏗️ System Architecture

* **Language:** Rust
* **Framework:** Axum (HTTP server & routing)
* **Async Runtime:** Tokio
* **Serialization:** Serde
* **Storage:** In-memory `HashMap` protected by `Mutex`
* **Server Port:** `3000`

### Exposed Endpoints

| Endpoint     | Description                                           |
| ------------ | ----------------------------------------------------- |
| `/authorize` | Handles ISO 8583 authorization requests (0100 → 0110) |
| `/reversal`  | Handles ISO 8583 reversal requests (0400 → 0410)      |

---

## 🔄 Authorization Flow (`/authorize`)

### Incoming Request

* MTI: **0100** (Authorization Request)
* PAN (Card Number)
* Transaction Amount
* Date & Time
* STAN (System Trace Audit Number)
* Merchant & Currency Information

### Business Logic

* Request is validated for correct MTI
* **Approval Rule:**

  * PAN starting with **4** → Approved
  * Any other PAN → Rejected

### On Approval

* Transaction is stored in memory
* Indexed using **STAN** as the unique key

### Response

* MTI: **0110** (Authorization Response)
* ISO Response Codes:

  * `00` → Approved
  * `05` → Declined
* Echoes relevant request fields
* Includes a human-readable message

---

## 🔁 Reversal Flow (`/reversal`)

### Incoming Request

* MTI: **0400** (Reversal Request)
* Original STAN
* Original PAN & Amount

### Processing Logic

* MTI validation
* System checks whether the original transaction exists

### Response

* MTI: **0410** (Reversal Response)
* Approved if original transaction is found
* Rejected if transaction does not exist
* Original details echoed back with descriptive message

---

## 🧠 Data Storage & State Management

* Transactions stored in an in-memory `HashMap`
* Structure: `STAN → Transaction Details`
* Access synchronized using a `Mutex`
* Ensures **thread safety** for concurrent API calls

---

## 🧪 Sample API Requests

### ▶ Authorization Request (0100)

```json
{
  "mti": "0100",
  "pan": "4123456789012345",
  "amount": 2500,
  "currency": "INR",
  "stan": "123456",
  "transaction_datetime": "2026-01-22T10:30:00",
  "merchant_id": "MERCHANT123"
}
```

### ▶ Authorization Response (0110 – Approved)

```json
{
  "mti": "0110",
  "response_code": "00",
  "stan": "123456",
  "message": "Transaction Approved"
}
```

---

### ▶ Reversal Request (0400)

```json
{
  "mti": "0400",
  "original_stan": "123456",
  "pan": "4123456789012345",
  "amount": 2500
}
```

### ▶ Reversal Response (0410 – Approved)

```json
{
  "mti": "0410",
  "response_code": "00",
  "original_stan": "123456",
  "message": "Reversal Successful"
}
```

---

## ▶ Running the Project Locally

```bash
cargo run
```

Server starts at:

```
http://localhost:3000
```

Use **Postman** or **Thunder Client** to test the APIs.

---

## ⚠️ Scope & Limitations

* ❌ No real payment network integration
* ❌ No encryption, certificates, or key management
* ❌ No persistent database
* ❌ No PCI-DSS compliance

This project is **strictly for educational purposes**.

---

## 🎯 Learning Outcomes

* Understanding ISO 8583 MTIs & response codes
* Backend transaction authorization logic
* Reversal handling
* Thread-safe state management in Rust
* Fintech backend flow simulation

---

## 👤 Author

**Lisa Das**
Product Engineer (Fintech)
GitHub: [https://github.com/lisadascse72](https://github.com/lisadascse72)

---

## 📄 License

This project is open for learning and demonstration purposes.
