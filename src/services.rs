use crate::models::{AppState, SheetMetadata};
use log::{error, info};
use std::collections::HashMap;

pub async fn get_sheet_name(
    id: &str,
    sheet_name: &str,
    data: &AppState,
) -> Result<String, (String, u16)> {
    let sheet = urlencoding::decode(sheet_name.replace("+", " ").as_str())
        .unwrap_or_else(|_| sheet_name.replace("+", " ").into())
        .to_string();

    if let Ok(num) = sheet.parse::<i32>() {
        if num == 0 {
            return Err(("Sheet number cannot be 0".into(), 400));
        }

        let metadata_url = format!(
            "https://sheets.googleapis.com/v4/spreadsheets/{}?key={}",
            id, data.google_api_key
        );

        info!("Fetching sheet metadata for ID: {}", id);
        let metadata_response = match data.client.get(&metadata_url).send().await {
            Ok(resp) => resp,
            Err(e) => {
                error!("Failed to fetch sheet metadata: {}", e);
                return Err((format!("Failed to fetch sheet metadata: {}", e), 500));
            }
        };

        let metadata: SheetMetadata = match metadata_response.json().await {
            Ok(data) => data,
            Err(e) => {
                error!("Failed to parse sheet metadata: {}", e);
                return Err((format!("Failed to parse sheet metadata: {}", e), 500));
            }
        };

        if let Some(error) = metadata.error {
            let status = error
                .status
                .and_then(|s| s.parse::<u16>().ok())
                .unwrap_or(400);
            return Err((error.message, status));
        }

        let sheet_index = num - 1;
        match metadata.sheets.get(sheet_index as usize) {
            Some(sheet) => Ok(sheet.properties.title.clone()),
            None => Err((format!("There is no sheet number {}", num), 404)),
        }
    } else {
        Ok(sheet)
    }
}

pub fn process_sheet_data(values: &[Vec<String>]) -> Vec<HashMap<String, String>> {
    if values.is_empty() {
        return Vec::new();
    }

    let headers = &values[0];
    let mut rows = Vec::with_capacity(values.len() - 1);

    for row in values.iter().skip(1) {
        let mut row_data = HashMap::new();
        for (i, item) in row.iter().enumerate() {
            if i < headers.len() {
                row_data.insert(headers[i].clone(), item.clone());
            }
        }
        rows.push(row_data);
    }

    rows
}
