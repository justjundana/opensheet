# Changelog

## [1.1.0] - 2025-04-23

### Added

- **Custom Range Query Support**:  
  Clients can now specify a custom cell range using the `?range=` query parameter (e.g., `?range=A2:F100`) when requesting sheet data.  
  If the query is not provided, the default range `A1:ZZ` is used automatically.  
  This gives clients more control and improves performance when working with large spreadsheets.

- **List Sheets Endpoint**:  
  A new endpoint `GET /{spreadsheet_id}/sheets` is available to list all sheet names in a given spreadsheet.  
  Useful for dynamically determining available sheets without needing to know their names or positions in advance.

---

## [1.0.0] - 2025-04-23

### Added

- **Initial release** of OpenSheet, a lightweight API server that converts Google Sheets into a JSON API.
- **Instant JSON API**: Convert any Google Sheet into a JSON API with zero configuration.
- **Intelligent Caching**: Reduce API calls and improve response times with built-in caching.
- **Rate Limiting**: Configurable rate limits to protect the service from abuse.
- **CORS Support**: Built-in Cross-Origin Resource Sharing (CORS) support for web applications.
- **Sheet Selection**: Access sheets by name or index number.
- **Error Handling**: Comprehensive error responses with meaningful status codes.
- **Health Check Endpoint**: A dedicated endpoint to monitor the health of the service.
- **Environment Configuration**: Easy configuration via environment variables like API key, port, cache TTL, rate limits, and request timeouts.
- **Setup Instructions**: Clear and easy-to-follow installation steps in the README.
- **Directory Structure**: Logical directory structure with separate files for config, handlers, models, services, and utilities.
