use serde::Serialize;
use std::path::Path;
use crate::error::{Result, XlsxError};

#[derive(Debug, Serialize)]
pub struct OutputJson {
    pub headers: Vec<String>,
    pub types: Vec<String>,
    pub data: Vec<Vec<serde_json::Value>>,
}

impl OutputJson {
    pub fn new(headers: Vec<String>, types: Vec<String>, data: Vec<Vec<serde_json::Value>>) -> Self {
        Self {
            headers,
            types,
            data,
        }
    }

    pub fn save<P: AsRef<Path>>(&self, path: P, pretty: bool) -> Result<()> {
        let json: String;
        if pretty {
            json = serde_json::to_string_pretty(self)?;
        } else {
            json = serde_json::to_string(self)?;
        }
        std::fs::write(path, json)?;
        Ok(())
    }
}

pub fn save_json<P: AsRef<Path>>(
    path: P,
    headers: Vec<String>,
    types: Vec<String>,
    data: Vec<Vec<serde_json::Value>>,
    pretty: bool,
) -> Result<()> {
    let output = OutputJson::new(headers, types, data);
    output.save(path, pretty)
}

pub fn ensure_output_dirs<P: AsRef<Path>>(output_dir: P) -> Result<()> {
    let server_dir = output_dir.as_ref().join("server");
    let client_dir = output_dir.as_ref().join("client");

    std::fs::create_dir_all(&server_dir)
        .map_err(|e| XlsxError::Io(e))?;
    std::fs::create_dir_all(&client_dir)
        .map_err(|e| XlsxError::Io(e))?;

    Ok(())
}

pub fn get_output_paths<P: AsRef<Path>>(
    output_dir: P,
    base_name: &str,
) -> (std::path::PathBuf, std::path::PathBuf) {
    let server_path = output_dir.as_ref().join("server").join(format!("{}.json", base_name));
    let client_path = output_dir.as_ref().join("client").join(format!("{}.json", base_name));
    (server_path, client_path)
} 