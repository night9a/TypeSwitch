# TypeSwitch

A fast, minimal Flask app for converting image file types, using Redis for metadata and API key management. Files are auto-deleted after 10 minutes.

## Features
- Convert between PNG, JPG, JPEG
- Simple API key authentication (no Flask-Login)
- Redis for metadata and API keys
- Files auto-delete after 10 minutes
- Minimal dependencies, high performance

## Setup

1. **Install dependencies:**
   ```bash
   pip install -r requirements.txt
   ```
2. **Start Redis:**
   Ensure Redis is running on `localhost:6379` (default).
   ```bash
   redis-server
   ```
3. **Run the app:**
   ```bash
   python app.py
   ```

## API Usage

### 1. Login
- **POST** `/login`
- Body (JSON): `{ "username": "admin", "password": "password123" }`
- Response: `{ "api_key": "..." }`

### 2. Convert File
- **POST** `/convert`
- Headers: `X-API-Key: <api_key>`
- Form Data: `file` (upload), `target_type` (e.g., `png`, `jpg`)
- Response: `{ "file_id": "...", "download_url": "/download/<file_id>" }`

### 3. Download File
- **GET** `/download/<file_id>`
- Headers: `X-API-Key: <api_key>`
- Response: File download

### 4. Check Status
- **GET** `/status/<file_id>`
- Headers: `X-API-Key: <api_key>`
- Response: `{ "status": "ready", "expires_in": 540 }`

### 5. Usage Info
- **GET** `/usage`
- Response: API usage instructions

## Notes
- Only image conversion (PNG, JPG, JPEG) is supported in this version.
- Files are deleted 10 minutes after upload/conversion.
- Default user: `admin` / `password123` 