use super::super::constant::*;
use serde_derive::{Deserialize, Serialize};
use std::{collections::HashMap, path::Path};
use tokio::fs;

#[derive(Debug, Deserialize, Serialize)]
pub struct Data {
    pub data: HashMap<String, Vec<String>>,
}

// 初始化文件，检查目录，读取所有json文件，解析为结构体
pub async fn setup_file() -> Result<Data, Box<dyn std::error::Error>> {
    let dir_path = Path::new(DEFAULT_DIRECTORY);
    if !dir_path.exists() {
        // 创建目录
        fs::create_dir(DEFAULT_DIRECTORY).await?;
    }

    let datas = read_json_in_dir(dir_path).await?;

    Ok(datas)
}

// 异步函数，用于读取目录中的所有JSON文件并解析它们
async fn read_json_in_dir(path: &Path) -> Result<Data, Box<dyn std::error::Error>> {
    let mut datas = HashMap::new();
    // 使用tokio::fs::read_directory_entry来异步读取目录项
    let mut stream = tokio::fs::read_dir(path).await?;

    while let Some(entry) = stream.next_entry().await? {
        let path = entry.path();

        if path.extension() == Some("json".as_ref()) {
            // 异步读取并解析JSON文件
            let data = read_and_parse_json_file(&path).await?;
            for (k, v) in data {
                datas.insert(k, v);
            }
        }
    }
    Ok(Data { data: datas })
}

async fn read_and_parse_json_file(
    path: &Path,
) -> Result<HashMap<String, Vec<String>>, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(path).await;
    match content {
        Ok(content) => {
            let data: HashMap<String, Vec<String>> = serde_json::from_str(&content)?;
            Ok(data)
        }
        Err(e) => {
            println!("{}", e);
            Err(Box::new(e))
        }
    }
}

#[tokio::test]
async fn test_read_json_in_dir() {
    let path = Path::new(DEFAULT_DIRECTORY);
    let datas = read_json_in_dir(path).await;
    assert_eq!(true, datas.is_ok());
    let data = datas.unwrap();
    let data = data.data;
    println!("data: {:?}", data);
    assert_eq!(2, data.len());
    assert_eq!(true, data.contains_key("bitFlag"));
    assert_eq!(true, data.contains_key("standby3"));
}

#[tokio::test]
async fn test_read_and_parse_json() {
    let path = Path::new(DEFAULT_DIRECTORY);
    if !path.exists() {
        fs::create_dir(path).await.unwrap();
    }
    let mut files = tokio::fs::read_dir(path).await.unwrap();
    while let Some(file) = files.next_entry().await.unwrap() {
        println!("starting reading file[{}]", file.path().display());
        let data = read_and_parse_json_file(&file.path()).await;
        assert_eq!(true, data.is_ok());
        let data = data.unwrap();
        println!("data: {:?}", data);
        assert_eq!(2, data.len());
        assert_eq!(true, data.contains_key("bitFlag"));
        assert_eq!(true, data.contains_key("standby3"));
    }
}

#[tokio::test]
async fn test_json() {
    let json = r#"
    {
        "bitflag": ["是否通知", "是否通知成功"],
        "bitflag2": ["是否发送", "是否发送成功"],
        "bitflag3": ["新属性", "新属性的另一个值"]
    }
    "#;
    let data: HashMap<String, Vec<String>> = serde_json::from_str(json).unwrap();
    println!("{:?}", data)
}
