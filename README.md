# ISO 8583 Mastercard Mock API (Rust)

## 📌 Project Overview

This project is a **mock payment processing backend API** that simulates **Mastercard ISO 8583–based authorization and reversal flows** using **Rust** and the **Axum** web framework.

⚠️ **Important:** This is an **API service**, not a website. Use **Postman** or **Thunder Client** to send JSON requests and receive responses.  

It is designed for **learning, demonstration, and interview preparation**, helping developers understand how card-based financial transactions are processed at the backend—without connecting to any real bank, NPCI, or Mastercard network.

The API accepts **JSON-formatted ISO 8583–like messages**, where each field represents a specific **ISO 8583 Data Element (DE)**.

---

## 🌐 Base URL (Deployed)

https://iso8583-mastercard-mock-api-2.onrender.com

---

## 🏗️ System Architecture

* **Language:** Rust  
* **Framework:** Axum (HTTP server & routing)  
* **Async Runtime:** Tokio  
* **Serialization:** Serde  
* **Storage:** In-memory `HashMap` protected by `Mutex`  
* **Server Port (Local):** `3000`  

### Exposed Endpoints

| Endpoint     | Method | Description                                           |
| ------------ | ------ | ----------------------------------------------------- |
| `/authorize` | POST   | Handles ISO 8583 authorization requests (0100 → 0110) |
| `/reversal`  | POST   | Handles ISO 8583 reversal requests (0400 → 0410)      |

---

## 🔄 Authorization Flow (`/authorize`)

**Logic:**

* Request is validated for correct MTI (`0100`)  
* Approval Rule:  
  * PAN starting with **4** → Approved (`00`)  
  * Any other PAN → Declined (`05`)  
* Approved transactions are stored in-memory, indexed by **STAN**  
* Response MTI: `0110`  
* ISO Response Codes: `00` (Approved) or `05` (Declined)  
* Echoes request fields and includes a human-readable message  

---

## 🔁 Reversal Flow (`/reversal`)

**Logic:**

* Request is validated for correct MTI (`0400`)  
* Checks whether the original transaction exists (by **STAN**)  
* Response MTI: `0410`  
* ISO Response Codes: `00` (Approved) or `94` (Original transaction not found / duplicate)  
* Echoes original details with a human-readable message  

---

## 🧪 Postman / Thunder Client Usage

* This API is **meant for backend testing**, not a website.  
* Open **Postman** or **Thunder Client**  
* Send POST requests to the deployed endpoints:  

https://iso8583-mastercard-mock-api-2.onrender.com/authorize
https://iso8583-mastercard-mock-api-2.onrender.com/reversal


* Include JSON-formatted ISO 8583–like payloads in the body of requests  
* Observe responses directly in the client  

---

## ▶ Running Locally


cargo run
Server starts at:

```bash
http://localhost:3000
```
Use Postman or Thunder Client to test locally.
