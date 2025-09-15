
# Python Benchmark for xlsx2json-rs

这是一个用于测试和对比 xlsx2json-rs 项目性能的 Python benchmark 工具。

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

## 说明

- 此工具用于测试 Excel 文件转换为 JSON 的性能
- 可以与 Rust 版本的 xlsx2json-rs 进行性能对比
