# OpenSheet - Google Sheets API Server

A lightweight, high-performance API server that transforms Google Sheets into a simple RESTful JSON API. Built with Rust, this service allows you to use Google Sheets as a lightweight database or CMS with minimal setup.

## 📝 Project Description

OpenSheet enables developers to access Google Sheets data via a simple HTTP API. It converts spreadsheet data into well-structured JSON responses, making it perfect for prototyping, simple backend storage needs, or content management. By caching responses and implementing rate limiting, it ensures efficient performance even under load while respecting Google API quotas.

## ✨ Key Features

- **Instant JSON API**: Convert any Google Sheet into a JSON API with zero configuration
- **Intelligent Caching**: Reduce API calls and improve response times with built-in caching
- **Rate Limiting**: Protect your service from abuse with configurable rate limits
- **CORS Support**: Built-in Cross-Origin Resource Sharing for web applications
- **Sheet Selection**: Access sheets by name or index number
- **Error Handling**: Comprehensive error responses with meaningful status codes
- **Health Endpoint**: Monitor service health with dedicated health check endpoint
- **Environment Configuration**: Easily configurable via environment variables

## 🛠 Technology Stack

- **Language**: Rust
- **Framework**: Actix-Web
- **Async Runtime**: Tokio
- **Core Libraries**:
  - [`reqwest`](https://docs.rs/reqwest) – for HTTP requests
  - [`serde`](https://docs.rs/serde) – for serialization/deserialization
  - [`governor`](https://docs.rs/governor) – for rate limiting
  - [`anyhow`](https://docs.rs/anyhow) – for error handling

## 🚀 Installation Guide

### Prerequisites

- **A Google Cloud Platform account with the Sheets API enabled**  
  To use the Google Sheets API, you need to create an account on [Google Cloud Platform](https://console.cloud.google.com/), and then enable the [Google Sheets API](https://console.cloud.google.com/flows/enableapi?apiid=sheets.googleapis.com).

- **A Google API key with access to the Sheets API**  
  After enabling the Google Sheets API, you'll need to create an **API Key** that your application can use. Follow the instructions on this page to [obtain your Google API Key](https://console.cloud.google.com/apis/credentials) and configure it for your project.

### Setup Instructions

1. **Clone the repository**

```bash
git clone https://github.com/justjundana/opensheet.git
cd opensheet
```

2. **Create a `.env` file with the following variables**

```
GOOGLE_API_KEY=your_google_api_key
PORT=8080
HOST=localhost
CACHE_TTL_SECONDS=60
REQUEST_TIMEOUT_SECONDS=10
RATE_LIMIT_PER_MINUTE=60
```

3. **Build the project**

```bash
cargo build --release
```

4. **Run the server**

```bash
./target/release/opensheet
```

Or directly with cargo:

```bash
cargo run --release
```

## 📡 Usage Instructions

### Accessing Spreadsheet Data

The API follows this URL pattern:

```
GET http://localhost:8080/{spreadsheet_id}/{sheet_name}
```

Where:

- `spreadsheet_id` is the ID of your Google Sheet (found in the URL of your sheet)
- `sheet_name` is either the name of the sheet or its index number (starting at 1)

### Examples

**Access a sheet by name:**

```
GET http://localhost:8080/1wXNAh6PVCDf5iSqKIhf1AwTH_GPtW_hjlnaFamEI-7o/Sheet1
```

**Access a sheet by index:**

```
GET http://localhost:8080/1wXNAh6PVCDf5iSqKIhf1AwTH_GPtW_hjlnaFamEI-7o/1
```

**Note:** The first sheet is indexed starting from **1**, not **0**.

### Response Format

✅ Successful responses have this structure:

```json
{
  "transaction_code": "uuid-v4-string",
  "status": 200,
  "data": [
    {
      "header1": "value1",
      "header2": "value2"
    },
    {
      "header1": "value3",
      "header2": "value4"
    }
  ]
}
```

❌ Error responses look like this:

```json
{
  "transaction_code": "uuid-v4-string",
  "status": 404,
  "error": "Sheet not found"
}
```

### Health Check

```
GET http://localhost:8080/health
```

Response:

```json
{
  "status": "ok",
  "timestamp": "2025-04-22T12:34:56.789Z"
}
```

## ⚙️ Environment Variables

| Variable                  | Description                                  | Default    |
| ------------------------- | -------------------------------------------- | ---------- |
| `GOOGLE_API_KEY`          | Your Google API key                          | (Required) |
| `PORT`                    | Port to run the server on                    | 8080       |
| `HOST`                    | Host address to bind to                      | 127.0.0.1  |
| `CACHE_TTL_SECONDS`       | Time to live for cached responses in seconds | 60         |
| `REQUEST_TIMEOUT_SECONDS` | Timeout for Google API requests in seconds   | 10         |
| `RATE_LIMIT_PER_MINUTE`   | Maximum requests per minute per IP           | 60         |

## 📁 Directory Structure

```
opensheet/
├── src/
│   ├── main.rs         # Entry point
│   ├── config.rs       # Environment configuration
│   ├── handlers.rs     # Route handlers
│   ├── models.rs       # Data models and structs
│   ├── services.rs     # Business logic
│   └── utils.rs        # Helpers for responses
├── .env                # Environment variables
├── Cargo.toml          # Dependencies and metadata
└── README.md
```

## ⚡ Performance Notes

- The service implements memory caching to reduce API calls to Google
- Rate limiting helps prevent abuse and ensures service stability
- For production use, consider setting up a reverse proxy (like Nginx) in front of the service

## 🤝 Contributing Guidelines

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'feat: Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

Please ensure your code passes all tests and follows the Rust style guidelines.

## 📄 License

This project is licensed under the MIT License
