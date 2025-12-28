use reqwest;
use std::fs;
use tokio;
use futures::future::join_all;
use scraper::{Html, Selector};
use std::error::Error;
use std::fs::File;
use std::io::Write;
use base64::Engine as _;
use base64::engine::general_purpose::STANDARD;

fn main() {
    let _ = download_images("https://img5.qy0.ru/data/2506/47/{num}.jpg".to_string(), 1, 200, r"D:\Downloads\[HERESY (林檎蜜紀)] ".to_string(), 3);
}

/// 异步下载url图片
#[tokio::main]
async fn download_images(src_ori_img: String, start_num: i32, end_num: i32, download_dir: String, placeholder: i32) -> Result<(), Box<dyn Error>> {
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(30))
        .pool_max_idle_per_host(10)
        .build()?;

    fs::create_dir_all(&download_dir)?;

    // 使用带重试的下载函数
    let tasks: Vec<_> = (start_num..=end_num).map(|num| {
        let client = client.clone();
        let download_dir = download_dir.to_string();
        let formatted_num = format!("{:0width$}", num, width = placeholder as usize);
        let src_img = src_ori_img.replace("{num}", &formatted_num);
        async move {
            download_with_retry(client, num, &download_dir, 10, src_img).await
        }
    }).collect();

    let results = join_all(tasks).await;

    let success_count = results.iter().filter(|r| r.is_ok()).count();
    let failed_count = results.len() - success_count;

    println!("下载完成: 成功 {} 个, 失败 {} 个", success_count, failed_count);

    // 打印失败的项目编号
    if failed_count > 0 {
        let failed_nums: Vec<usize> = results
            .iter()
            .enumerate()
            .filter(|(_, r)| r.is_err())
            .map(|(i, _)| i + 1) // 转换回原始编号
            .collect();
        println!("失败的项目: {:?}", failed_nums);
    }

    Ok(())
}


/// 异步下载单个图片
async fn download_with_retry(
    client: reqwest::Client,
    num: i32,
    download_dir: &str,
    max_retries: usize,
    src_img: String
) -> Result<(), Box<dyn Error>> {
    let mut attempts = 0;
    let mut delay = std::time::Duration::from_millis(500); // 初始延迟 0.5 秒
    loop {
        match download_single_image(client.clone(), num, download_dir, src_img.clone()).await {
            Ok(_) => return Ok(()), // 成功则返回
            Err(e) => {
                attempts += 1;
                if attempts >= max_retries {
                    return Err(format!("图片 {} 经过 {} 次重试后仍然失败: {}", num, max_retries, e).into());
                }

                eprintln!("图片 {} 第 {} 次尝试失败: {}, {}秒后重试", num, attempts, e, delay.as_secs_f64());
                tokio::time::sleep(delay).await;
                delay = std::time::Duration::from_secs(delay.as_secs() * 2); // 指数退避
            }
        }
    }
}

/// 单个图片下载
async fn download_single_image(
    client: reqwest::Client,
    num: i32,
    download_dir: &str,
    src_img: String
) -> Result<(), Box<dyn Error>> {
    let url = src_img;
    let file_path = format!(r"{}\{}.webp", download_dir, num);

    let response = client.get(url).send().await?;

    if response.status() != reqwest::StatusCode::OK {
        return Err(format!("HTTP {}", response.status()).into());
    }

    let image_data = response.bytes().await?;
    tokio::fs::write(&file_path, &image_data).await?;

    Ok(())
}


/// 保存 解析base64图片url并解析 Base64图片以及保存
fn save_base64_image(base64_str: &str, output_path: &str) -> Result<(), Box<dyn Error>> {
    // 移除 data:image/jpeg;base64, 前缀
    let base64_data = if let Some(index) = base64_str.find("base64") {
        &base64_str[index + 7..]
    } else {
        base64_str
    };

    // 解码 Base64
    let decoded_data = STANDARD.decode(base64_data)?;

    // 保存到文件
    let mut file = File::create(output_path)?;
    file.write_all(&decoded_data)?;

    println!("图片已保存到: {}", output_path);
    println!("文件大小: {} 字节", decoded_data.len());

    Ok(())
}

/// 向后解析页面 并获取图片
///
fn get_php_page_img(url: String, start_page: i32, end_page: i32, id_name: String, id_host_name: String, tail: String) -> Result<(), Box<dyn Error>> {

    let (id, clear_url) = if let Some(index) = url.find(&id_name) {
        (&url[index..], &url[..index])
    }else{
        (url.as_str(), url.as_str())
    };
    // &page=3&host_id=0
    for page in start_page..=end_page {
        if page != start_page {
            let url_next = format!("{}&page={}{}=0{}{}", clear_url, page, id_host_name,id,  tail);
            match get_url_images_sec(url_next){
                Ok(_) => { println!("获取{page}页面图片成功")},
                Err(e) => {
                    println!("获取页面图片失败: {}", e);
                }
            };
        } else{
            match get_url_images_sec(url.clone().to_string()){
                Ok(_) => { println!("获取{page}页面图片成功")},
                Err(e) => {
                    println!("获取页面图片失败: {}", e);
                }
            };
        }
        println!("正在获取页面: {}", &url);
    }
    Ok(())
}


/// 获取页面图片
/// 获取页面图片
#[tokio::main]
async fn get_url_images_sec(url: String) -> Result<(), Box<dyn Error>> {
    let domain_name = if let Some(index) = url.find("index") {
        &url[index..]
    } else {
        &url
    };

    // 1. 获取HTML内容

    // 修复：使用配置好的客户端，添加User-Agent
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(30))
        .pool_max_idle_per_host(10)
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/143.0.0.0 Safari/537.36 Edg/143.0.0.0")
        .build()?;


    let response = client.post(&url)
        .header("Accept", "text/html,application/xhtml+xml,application/xml;q=0.9,image/webp,*/*;q=0.8")
        .header("Accept-Language", "zh-CN,zh;q=0.9,en;q=0.8")
        .header("Accept-Encoding", "gzip, deflate")
        .header("Connection", "keep-alive")
        .header("Upgrade-Insecure-Requests", "1")
        .send().await?;

    println!("尝试获取页面");

    // 检查响应状态
    if response.status().is_success() {
        let html_content = response.text().await?;
        println!("获取成功，HTML长度: {} 字符", html_content.len());

        // 2. 解析HTML
        let document = Html::parse_document(&html_content);

        // 3. 创建CSS选择器
        let img_selector = Selector::parse("img")?;

        // 4. 提取所有img标签的src
        let mut image_urls = Vec::new();

        for element in document.select(&img_selector) {
            if let Some(src) = element.value().attr("src") {
                image_urls.push(src.to_string());
                println!("找到图片: {:?}", src.get(..30));
            }

            // // 也可以检查其他属性，比如data-src（懒加载图片）
            // if let Some(data_src) = element.value().attr("data-src") {
            //     println!("找到懒加载图片: {}", data_src);
            //     image_urls.push(data_src.to_string());
            // }
        }

        println!("\n总共找到 {} 张图片", image_urls.len());

        // 5. 处理相对路径（可选）
        for img_url in &image_urls {
            let full_url = if img_url.starts_with("http") {
                img_url.clone()
            } else if img_url.starts_with("/") {
                // 基于域名的相对路径
                format!("{}{}", domain_name, img_url)
            } else {
                // 其他相对路径
                format!("{}{}", domain_name, img_url)
            };
            println!("完整URL: {}", full_url);
        }
    } else {
        eprintln!("请求失败，状态码: {}", response.status());
        return Err(format!("HTTP {}", response.status()).into());
    }

    Ok(())
}
