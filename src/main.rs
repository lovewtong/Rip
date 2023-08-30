use epub;

fn main() {
    
    let path_to_epub = "123.epub";
    let mut book = epub::doc::EpubDoc::new(path_to_epub).expect("Failed to open EPUB file");
    println!("EPUB file loaded successfully.");

    // 打印元数据
    let title = book
        .metadata
        .get("title")
        .and_then(|v| v.get(0))
        .cloned()
        .unwrap_or_else(|| String::from("Unknown"));
    let authors = book
        .metadata
        .get("creator")
        .and_then(|v| v.get(0))
        .cloned()
        .unwrap_or_else(|| String::from("Unknown"));

    println!("Title: {:?}", title);
    println!("Metadata: {:?}", book.metadata);
    println!("Authors: {:?}", authors);

    // 打印每章节的内容
    let items: Vec<_> = book.spine.clone();
    println!("Number of spine items: {}", items.len());

    for item_id in items.iter() {
        let path_option = book.resources.get(item_id).map(|(p, _)| p.clone());
        if let Some(path) = path_option {
            if let Some(resource) = book.get_resource_by_path(&path) {
                let content_as_str =
                    std::str::from_utf8(&resource).expect("Failed to convert to string");
                println!("---\n{}", content_as_str);
            } else {
                println!("Warning: Failed to get resource for item: {:?}", item_id);
            }
        }
    }

    // Print the resources in the EPUB file
    println!("\nResources in the EPUB file:");
    for (resource_id, (path, mime)) in &book.resources {
        println!("ID: {}, Path: {:?}, MIME: {}", resource_id, path, mime);
    }
}
