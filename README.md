# Excel转JSON工具 (xlsx2json) - Rust实现

这是使用 Rust 语言实现的 Excel 转 JSON 工具，利用了 Rust 的类型系统和并发特性。

这个项目最初是为了测试 Cursor 的代码生成能力而创建的。当时我完全不懂 Rust，现在基本能看懂 Rust 代码，也能写一些简单的代码。经过人工 review 之后，我把这个项目开源出来。

这个项目的特点是足够简单和足够快。通过简单的 Excel 格式约定，没有复杂的设计，直接转成 JSON 文件。复杂的功能在获得 JSON 之后再做，比如代码生成。

**性能对比**：Python 处理单个 Excel 文件（3列数据，2万行）需要 0.4054 秒，而 Rust 只需要 0.076 秒，性能提升约 **5.3倍**。在更复杂的场景下（多个文件），Rust 的优势会更加明显。

## 功能特点

- 支持处理单个Excel文件或目录下的所有Excel文件
- 自动识别Excel文件结构（标记行、类型说明、表头、数据）
- 支持服务端/客户端数据分离（通过标记行指定）
- 支持自定义JSON输出文件名（通过TOML配置）
- 使用 Rayon 进行并行处理，提高转换效率
- 智能跳过未修改的文件
- 详细的错误报告和统计信息
- 使用 Rust 的类型系统确保数据安全
- 使用 Rust 的错误处理机制提供更好的错误信息

## 安装

1. 确保已安装 Rust 环境
2. 克隆或下载本项目
3. 在项目目录下运行：
```bash
cargo build --release
```

## 使用方法

参考 `example` 目录，我已经提供了原型项目的若干配置表和可执行文件，可以直接使用，无需安装 Rust 环境。

### 基本用法

```bash
# 处理单个文件
xlsx2json -i input.xlsx

# 处理目录下所有Excel文件
xlsx2json -i ./excel_files
```

### 命令行参数

- `-i, --input`: 输入Excel文件路径或目录
- `-d, --output-dir`: 输出根目录（默认为"output"）
- `-c, --config`: 配置文件路径（TOML格式）
- `-w, --workers`: 并行处理的工作线程数（默认为CPU核心数-1）
- `--pretty`: 美化JSON输出

### 配置文件和Excel文件格式要求

配置文件用于自定义JSON输出文件名，采用TOML格式。可以通过 `-c` 参数指定配置文件路径。

Excel文件需要按以下格式组织：

1. 第二行：标记行
   - `s`: 仅服务端数据
   - `c`: 仅客户端数据
   - `b`: 服务端和客户端都需要的数据

2. 第三行：数据类型说明
   - `int`: 整数类型
   - `float`: 浮点数类型
   - `bool`: 布尔类型
   - `string`: 字符串类型
   - `json`: JSON对象或数组

3. 第四行：表头名称

4. 第五行开始：数据内容

### 输出结构

程序会在输出目录下创建 `server` 和 `client` 两个子目录，分别存放服务端和客户端数据。

## License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
</sub>