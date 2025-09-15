use serde::Deserialize;
use std::path::Path;
use crate::error::{Result, XlsxError};

#[derive(Debug, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub file_mappings: std::collections::HashMap<String, String>,
}

impl Config {
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self> {
        let content = std::fs::read_to_string(path)
            .map_err(|e| XlsxError::Config(format!("无法读取配置文件: {}", e)))?;
        
        toml::from_str(&content)
            .map_err(|e| XlsxError::Config(format!("无法解析配置文件: {}", e)))
    }

    pub fn get_output_name(&self, excel_file: &str) -> String {
        // 获取Excel文件名（不含路径和扩展名）
        let base_name = std::path::Path::new(excel_file)
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or(excel_file);

        // 如果配置中有映射，使用映射的名称
        self.file_mappings
            .get(base_name)
            .cloned()
            .unwrap_or_else(|| base_name.to_string())
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            file_mappings: std::collections::HashMap::new(),
        }
    }
} 