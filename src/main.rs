use anyhow::{Context, Result};
use clap::Parser;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
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
        
        println!("正在转换 {}", jupyter_file.display());
        
        // 构建命令
        let status = Command::new("jupyter")
            .arg("nbconvert")
            .arg("--to")
            .arg("markdown")
            .arg(jupyter_file.to_str().unwrap())
            .arg("--output-dir")
            .arg(output_subdir.to_str().unwrap())
            .status()
            .context("执行jupyter nbconvert命令失败")?;
            
        if !status.success() {
            println!("转换 {} 时出错", jupyter_file.display());
        }
    }
    
    Ok(())
}
