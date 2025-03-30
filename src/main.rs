use anyhow::{Context, Result};
use clap::Parser;
use serde_json::Value;
use std::fs;
use std::io::Write;
use std::path::Path;
use walkdir::WalkDir;

/// 将Jupyter笔记本转换为Markdown文件的工具
#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    /// 包含Jupyter笔记本文件的目录
    #[arg(short = 'i', long = "input_dir")]
    input_dir: String,

    /// 保存Markdown文件的目录
    #[arg(short = 'o', long = "output_dir")]
    output_dir: String,
}

fn main() -> Result<()> {
    // 解析命令行参数
    let args = Args::parse();
    
    // 调用转换函数
    convert_jupyter_to_markdown(&args.input_dir, &args.output_dir)?;
    
    Ok(())
}

/// 将输入目录中的所有Jupyter笔记本转换为输出目录中的Markdown文件。
///
/// # 参数
/// * `input_dir` - 包含Jupyter笔记本文件的目录
/// * `output_dir` - 保存Markdown文件的目录
fn convert_jupyter_to_markdown(input_dir: &str, output_dir: &str) -> Result<()> {
    // 如果输出目录不存在，则创建
    fs::create_dir_all(output_dir).context("创建输出目录失败")?;
    
    let input_path = Path::new(input_dir);
    
    // 查找所有.ipynb文件
    let mut jupyter_files = Vec::new();
    for entry in WalkDir::new(input_dir) {
        let entry = entry.context("读取目录条目失败")?;
        if entry.file_type().is_file() && entry.path().extension().map_or(false, |ext| ext == "ipynb") {
            jupyter_files.push(entry.path().to_path_buf());
        }
    }
    
    if jupyter_files.is_empty() {
        println!("在 {} 中未找到Jupyter笔记本", input_dir);
        return Ok(());
    }
    
    println!("找到 {} 个需要转换的Jupyter笔记本", jupyter_files.len());
    
    // 处理每个笔记本
    for jupyter_file in jupyter_files {
        // 获取相对路径以保持目录结构
        let rel_path = jupyter_file.strip_prefix(input_path)
            .context("计算相对路径失败")?;
        
        // 在output_dir中创建子目录
        let output_subdir = Path::new(output_dir).join(rel_path.parent().unwrap_or_else(|| Path::new("")));
        fs::create_dir_all(&output_subdir).context("创建输出子目录失败")?;
        
        // 确定输出文件名：将.ipynb替换为.md
        let file_stem = jupyter_file.file_stem().unwrap().to_str().unwrap();
        let output_file_path = output_subdir.join(format!("{}.md", file_stem));
        
        println!("正在转换 {} 到 {}", jupyter_file.display(), output_file_path.display());
        
        // 读取ipynb文件
        let notebook_content = fs::read_to_string(&jupyter_file)
            .context(format!("读取文件失败: {}", jupyter_file.display()))?;
        
        // 解析JSON
        let notebook: Value = serde_json::from_str(&notebook_content)
            .context("解析Jupyter笔记本JSON失败")?;
        
        // 将笔记本转换为Markdown
        let markdown = convert_notebook_to_markdown(&notebook)
            .context("转换笔记本到Markdown失败")?;
        
        // 写入Markdown文件
        let mut file = fs::File::create(&output_file_path)
            .context(format!("创建Markdown文件失败: {}", output_file_path.display()))?;
        file.write_all(markdown.as_bytes())
            .context("写入Markdown内容失败")?;
    }
    
    println!("转换完成！");
    
    Ok(())
}

/// 将Jupyter笔记本JSON转换为Markdown字符串
fn convert_notebook_to_markdown(notebook: &Value) -> Result<String> {
    let mut markdown = String::new();
    
    // 尝试获取标题/元数据
    if let Some(metadata) = notebook["metadata"].as_object() {
        if let Some(title) = metadata.get("title") {
            if let Some(title_str) = title.as_str() {
                markdown.push_str(&format!("# {}\n\n", title_str));
            }
        }
    }
    
    // 处理单元格
    if let Some(cells) = notebook["cells"].as_array() {
        for cell in cells {
            let cell_type = cell["cell_type"].as_str().unwrap_or("code");
            
            match cell_type {
                "markdown" => {
                    // 直接添加Markdown内容
                    if let Some(source) = cell["source"].as_array() {
                        for line in source {
                            if let Some(text) = line.as_str() {
                                markdown.push_str(text);
                            }
                        }
                        markdown.push_str("\n\n");
                    } else if let Some(text) = cell["source"].as_str() {
                        markdown.push_str(text);
                        markdown.push_str("\n\n");
                    }
                },
                "code" => {
                    // 添加代码块
                    markdown.push_str("```python\n");
                    if let Some(source) = cell["source"].as_array() {
                        for line in source {
                            if let Some(text) = line.as_str() {
                                markdown.push_str(text);
                            }
                        }
                    } else if let Some(text) = cell["source"].as_str() {
                        markdown.push_str(text);
                    }
                    markdown.push_str("\n```\n\n");
                    
                    // 处理代码输出
                    if let Some(outputs) = cell["outputs"].as_array() {
                        for output in outputs {
                            if let Some(output_type) = output["output_type"].as_str() {
                                match output_type {
                                    "stream" => {
                                        markdown.push_str("输出:\n\n```\n");
                                        if let Some(text_array) = output["text"].as_array() {
                                            for line in text_array {
                                                if let Some(text) = line.as_str() {
                                                    markdown.push_str(text);
                                                }
                                            }
                                        } else if let Some(text) = output["text"].as_str() {
                                            markdown.push_str(text);
                                        }
                                        markdown.push_str("\n```\n\n");
                                    },
                                    "execute_result" | "display_data" => {
                                        // 处理输出数据
                                        if let Some(data) = output["data"].as_object() {
                                            // 处理文本/html输出
                                            if let Some(text_plain) = data.get("text/plain") {
                                                markdown.push_str("结果:\n\n```\n");
                                                if let Some(text_array) = text_plain.as_array() {
                                                    for line in text_array {
                                                        if let Some(text) = line.as_str() {
                                                            markdown.push_str(text);
                                                        }
                                                    }
                                                } else if let Some(text) = text_plain.as_str() {
                                                    markdown.push_str(text);
                                                }
                                                markdown.push_str("\n```\n\n");
                                            }
                                            
                                            // TODO: 如果需要，可以添加对图像等其他输出类型的处理
                                        }
                                    },
                                    _ => {}
                                }
                            }
                        }
                    }
                },
                _ => {}
            }
        }
    }
    
    Ok(markdown)
}
