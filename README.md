# Gender Classification API

A REST API built with Rust and Actix Web that predicts gender from a name using the Genderize API.  
It processes raw data, adds business logic, and returns a structured JSON response.

---

## 🚀 Features

- Classify gender from a given name
- Uses external Genderize API
- Renames and processes API fields
- Computes confidence score
- Returns structured JSON response
- Proper error handling (400, 422, 502, 500)
- CORS enabled for frontend access
- Production-ready structure
- Deployment-ready (Railway/Fly.io)

---

## 🛠 Tech Stack

- Rust
- Actix Web
- Reqwest (HTTP client)
- Serde (JSON serialization/deserialization)
- Chrono (timestamps)

---

## 📡 API Endpoint

### Classify Name
