use std::fs::File;
use std::io::{self, Read};
use std::time::Instant as StdInstant;
use tokio::fs::File as TokioFile;
use tokio::io::AsyncReadExt;
use tokio::time::Instant as TokioInstant;
use reqwest;
use anyhow::{Result, Context};

fn read_file_sync(path: &str) -> io::Result<String> {
    let mut file = File::open(path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}

fn make_request_sync(url: &str) -> Result<String> {
    let client = reqwest::blocking::Client::new();
    let response = client.get(url)
        .send()
        .with_context(|| format!("Failed to send request to {}", url))?
        .text()
        .context("Failed to get response text")?;
    Ok(response)
}

fn run_sync() -> Result<()> {
    let file_path = "test_file.txt";
    let urls = vec![
        "https://httpbin.org/get",
        "https://httpbin.org/delay/1",
        "https://httpbin.org/delay/2",
    ];

    let start = StdInstant::now();
    
    let content = read_file_sync(file_path)?;
    println!("Sync file read: {} bytes", content.len());
    
    for url in urls {
        let response = make_request_sync(url)?;
        println!("Sync request to {}: {} bytes", url, response.len());
    }

    let elapsed = start.elapsed();
    println!("Synchronous execution completed in: {:?}", elapsed);

    Ok(())
}

async fn read_file_async(path: &str) -> io::Result<String> {
    let mut file = TokioFile::open(path).await?;
    let mut content = String::new();
    file.read_to_string(&mut content).await?;
    Ok(content)
}

async fn make_request_async(url: &str) -> Result<String> {
    let client = reqwest::Client::new();
    let response = client.get(url)
        .send()
        .await
        .with_context(|| format!("Failed to send request to {}", url))?
        .text()
        .await
        .context("Failed to get response text")?;
    Ok(response)
}

async fn run_async() -> Result<()> {
    let file_path = "test_file.txt";
    let urls = vec![
        "https://httpbin.org/get",
        "https://httpbin.org/delay/1",
        "https://httpbin.org/delay/2",
    ];

    let start = TokioInstant::now();
    
    let file_path_owned = file_path.to_string();
    
    let file_task = tokio::spawn(async move {
        match read_file_async(&file_path_owned).await {
            Ok(content) => {
                println!("Async file read: {} bytes", content.len());
                Ok(content)
            },
            Err(e) => Err(anyhow::Error::new(e)),
        }
    });
    
    let request_tasks = urls.iter().map(|&url| {
        let url = url.to_string();
        tokio::spawn(async move {
            match make_request_async(&url).await {
                Ok(response) => {
                    println!("Async request to {}: {} bytes", url, response.len());
                    Ok(response)
                },
                Err(e) => Err(e),
            }
        })
    }).collect::<Vec<_>>();
    
    file_task.await??;
    
    for task in request_tasks {
        task.await??;
    }

    let elapsed = start.elapsed();
    println!("Asynchronous execution completed in: {:?}", elapsed);

    Ok(())
}

fn main() -> Result<()> {
    let file_path = "test_file.txt";
    if !std::path::Path::new(file_path).exists() {
        std::fs::write(file_path, "This is a test file content for the async/sync comparison")?;
    }
    
    println!("Running synchronous version...");
    run_sync()?;
    
    println!("\nRunning asynchronous version...");
    let rt = tokio::runtime::Runtime::new()?;
    rt.block_on(run_async())?;

    Ok(())
}