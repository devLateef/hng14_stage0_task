# Gender Classification API

A REST API built with Rust and Actix Web that predicts gender from a name using the Genderize API.  
It processes raw data, adds business logic, and returns a structured JSON response.

---

## Features

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

## API Endpoint

### Classify Name


---

## Example Request
- GET http://localhost:3000/api/classify?name=john

---

## Success Response (200 OK)

```json
{
  "status": "success",
  "data": {
    "name": "john",
    "gender": "male",
    "probability": 0.99,
    "sample_size": 1234,
    "is_confident": true,
    "processed_at": "2026-04-01T12:00:00Z"
  }
}

## Bad request (400 OK)

{
  "status": "error",
  "message": "Missing or empty name parameter"
}

## 422 Unprocessable Entity

{
  "status": "error",
  "message": "No prediction available for the provided name"
}

## 502 Bad Gateway

{
  "status": "error",
  "message": "Upstream service failure"
}

## 500 Internal Server Error
{
  "status": "error",
  "message": "Internal server error"
}
