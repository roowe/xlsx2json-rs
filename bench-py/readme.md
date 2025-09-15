
# Python Benchmark for xlsx2json-rs

这是一个用于测试和对比 xlsx2json-rs 项目性能的 Python benchmark 工具，没有完整实现，只是为了简单对比。

## 初始化

使用uv安装python环境
```
uv venv
source .venv/bin/activate  # Windows: .venv\Scripts\activate
uv pip install openpyxl
```

## 运行

```
uv run main.py
执行时间: 0.4054 秒
```

## 性能对比

Python版本执行时间: **0.4054 秒**

Rust版本执行时间: **75.5493ms (约0.076秒)**

```bash
0-xlsx2json.exe -i Z.资源.xlsx -c 0-config.toml
服务端数据已保存到: output\server\base_res.json
客户端数据已保存到: output\client\base_res.json
  [00:00:00] [########################################] 1/1 (0s)
批量处理完成！
成功处理文件数: 1
失败文件数: 0
并行工作线程数: 15
总执行时间: 75.5493ms
```

**性能提升**: Rust版本比Python版本快约 **5.3倍**，再加上并行处理和复杂JSON数据处理能力，Rust版本优势会更明显。

## 说明

- 此工具用于测试 Excel 文件转换为 JSON 的性能
- 可以与 Rust 版本的 xlsx2json-rs 进行性能对比
- 输出结果保存在 `output.json` 文件中
