# Jupyter2MD (Rust 版本)

将 Jupyter 笔记本(.ipynb)批量转换为 Markdown(.md)文件的 Rust 实现工具。

## 功能特点

- 递归查找目录中所有的 Jupyter 笔记本文件
- 保持原始目录结构
- 支持批量转换
- 提供简单的命令行接口

## 环境要求

- Rust 1.56+
- Cargo
- Jupyter nbconvert

## 安装

确保您已安装 Jupyter nbconvert:

```bash
pip install nbconvert
```

编译 Rust 程序:

```bash
cargo build --release
```

## 使用方法

使用 cargo 运行:

```bash
cargo run -- -i <输入目录> -o <输出目录>
```

或者编译后运行:
```bash
cargo build --release
./target/release/nb2md-rs -i <输入目录> -o <输出目录>
```

## 项目结构

```
nb2md-rs/
├── Cargo.toml        # 项目依赖配置
├── README.md         # 本文件
└── src/
    └── main.rs       # 主程序代码
```

## 许可证

本项目采用Apache License 2.0许可证。

## 作者

筱可 & 筱可AI研习社
