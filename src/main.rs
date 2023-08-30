use epub::doc::EpubDoc;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>>{
    
    // 打开epub文件
    let file_path = "123.epub";
    let mut file = File::open(file_path)?;

    // 读取文件内容
    let mut epub_content = Vec::new();
    file.read_to_end(&mut epub_content)?;

    // 创建EpubDoc对象
    let epub_doc = EpubDoc::new(&epub_content)?;

    // 获取元数据信息
    let metadata = epub_doc.metadata();

    // 打印元数据信息
    println!("Tittle is :{}",metadata.title);
    println!("Author is :{}",metadata.author);
    println!("Description is :{}",metadata.description);

    Ok(())
}
