use calamine::{Reader, Xlsx, open_workbook};
use std::path::Path;
use crate::error::{Result, XlsxError};
use crate::utils::{convert_value_by_type, filter_data};
use crate::json::save_json;

pub struct ExcelProcessor {
    config: crate::config::Config,
    output_dir: std::path::PathBuf,
    pretty: bool,
}

impl ExcelProcessor {
    pub fn new(config: crate::config::Config, output_dir: std::path::PathBuf, pretty: bool) -> Self {
        Self {
            config,
            output_dir,
            pretty,
        }
    }

    pub fn process_file<P: AsRef<Path>>(&self, excel_file: P) -> Result<()> {
        let excel_file = excel_file.as_ref();
        let output_name = self.config.get_output_name(
            excel_file.file_name()
                .and_then(|n| n.to_str())
                .ok_or_else(|| XlsxError::InvalidExcel("无效的文件名".to_string()))?
        );

        // 获取输出文件路径
        let (server_path, client_path) = crate::json::get_output_paths(&self.output_dir, &output_name);

        // 检查是否需要重新生成
        if !crate::utils::need_regenerate(excel_file, &server_path, &client_path)? {
            println!("跳过文件 {}: JSON文件已是最新", excel_file.display());
            return Ok(());
        }

        // 打开Excel文件
        let mut workbook: Xlsx<_> = open_workbook(excel_file)
            .map_err(|e| XlsxError::Excel(format!("无法打开Excel文件: {}", e)))?;

        // 获取第一个工作表
        let sheet_name = workbook.sheet_names()
            .first()
            .ok_or_else(|| XlsxError::Excel("Excel文件没有工作表".to_string()))?
            .clone();

        let range = workbook.worksheet_range(&sheet_name)
            .map_err(|e| XlsxError::Excel(format!("读取工作表时出错: {}", e)))?;

        // 获取所有行，写得比较粗暴，使用迭代器或许好一点
        let rows: Vec<Vec<String>> = range.rows()
            .map(|row| row.iter()
                .map(|cell| cell.to_string())
                .collect())
            .collect();

        if rows.len() < 4 {
            return Err(XlsxError::InvalidExcel("Excel文件至少需要包含标记行、类型说明、表头和数据行".to_string()));
        }

        // 获取标记行（第二行，索引为1）
        let marks: Vec<String> = rows[1].iter()
            .map(|s| s.trim().to_lowercase())
            .collect();

        // 获取类型说明（第三行，索引为2）
        let types: Vec<String> = rows[2].iter()
            .map(|s| s.trim().to_string())
            .collect();

        // 获取表头（第四行，索引为3）
        let headers: Vec<String> = rows[3].iter()
            .map(|s| s.trim().to_string())
            .collect();

        // 找到第一个空表头的位置
        let valid_header_count = headers.iter()
            .position(|h| h.is_empty())
            .unwrap_or(headers.len());

        // 截断到有效长度
        let headers = headers[..valid_header_count].to_vec();
        let types = types[..valid_header_count].to_vec();
        let marks = marks[..valid_header_count].to_vec();

        // 从第五行开始读取数据
        let mut data = Vec::new();
        let mut has_error = false;

        for (i, row) in rows.iter().skip(4).enumerate() {
            if row.is_empty() {
                continue;
            }

            let mut row_data = Vec::new();
            for (j, cell) in row.iter().take(valid_header_count).enumerate() {
                match convert_value_by_type(cell, &types[j], i + 5, &headers[j]) {
                    Ok(value) => row_data.push(value),
                    Err(e) => {
                        eprintln!("错误: {}", e);
                        has_error = true;
                        row_data.push(serde_json::Value::Null);
                    }
                }
            }
            data.push(row_data);
        }

        if has_error {
            return Err(XlsxError::Excel("转换过程中出现错误，请检查上述错误信息".to_string()));
        }

        // 分别生成服务端和客户端数据
        let (server_headers, server_types, server_data) = filter_data(&headers, &types, &marks, &data, "s");
        let (client_headers, client_types, client_data) = filter_data(&headers, &types, &marks, &data, "c");

        // 生成服务端数据
        if !server_headers.is_empty() {
            save_json(&server_path, server_headers, server_types, server_data, self.pretty)?;
            println!("服务端数据已保存到: {}", server_path.display());
        }

        // 生成客户端数据
        if !client_headers.is_empty() {
            save_json(&client_path, client_headers, client_types, client_data, self.pretty)?;
            println!("客户端数据已保存到: {}", client_path.display());
        }

        Ok(())
    }
} 