# nb2md-rs - Jupyter笔记本转换为Markdown工具

## 项目简介

nb2md-rs是一个简单而高效的Rust工具，用于将Jupyter笔记本(.ipynb文件)批量转换为Markdown格式(.md文件)。该工具采用纯Rust实现，无需依赖Python环境，能够保留Jupyter笔记本中的代码块、输出结果以及Markdown文本等内容。

## 功能特点

- 递归扫描目录，批量转换所有找到的Jupyter笔记本
- 保留原始目录结构
- 直接解析.ipynb文件的JSON内容
- 支持代码单元格和Markdown单元格的转换
- 纯Rust实现，无Python依赖
- 高效的文件处理性能

## 环境要求

- Rust 1.56+
- Cargo

## 安装方法

### 从源代码编译

```bash
# 克隆仓库
git clone https://github.com/li-xiu-qi/nb2md-rs.git
cd nb2md-rs

# 编译
cargo build --release
```

## 使用方法

### 命令行使用

```bash
cargo run -- -i <输入目录> -o <输出目录>
```

或者使用编译后的可执行文件：

```bash
./target/release/nb2md-rs -i <输入目录> -o <输出目录>
```

### 参数说明

- `-i, --input_dir`: 包含Jupyter笔记本(.ipynb文件)的目录
- `-o, --output_dir`: 保存转换后Markdown文件(.md文件)的目录

### 示例

```bash
cargo run -- -i ./jupyter_notebooks -o ./markdown_files
```

上述命令会将`./jupyter_notebooks`目录（包括所有子目录）中的所有.ipynb文件转换为Markdown格式，并保存到`./markdown_files`目录中，同时保持原始的目录结构。

## 转换过程

转换过程如下：

1. 扫描输入目录及其所有子目录，查找所有.ipynb文件
2. 对于每个找到的.ipynb文件：
   - 读取并解析文件内容（JSON格式）
   - 提取笔记本中的各个单元格（代码、Markdown等）
   - 将这些单元格转换为Markdown格式
   - 保存转换后的内容到对应的.md文件

## 项目结构

```
nb2md-rs/
├── Cargo.toml        # 项目依赖配置
├── README.md         # 本文件
└── src/
    └── main.rs       # 主程序代码
```

## 贡献指南

欢迎贡献！如果你有兴趣改进这个工具，可以通过以下方式参与：

1. Fork本仓库
2. 创建你的特性分支 (`git checkout -b feature/amazing-feature`)
3. 提交你的更改 (`git commit -m 'Add some amazing feature'`)
4. 推送到分支 (`git push origin feature/amazing-feature`)
5. 开启一个Pull Request

## 许可证

本项目采用BSD 3-Clause许可证 - 详情请参阅LICENSE文件

## 作者

筱可 & 筱可AI研习社
