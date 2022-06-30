//! Utilities to interact with the [`malshare`](https://malshare.com) API.

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;

async fn _get_api_call_limit_helper(api_key: &str, target: &str) -> Result<u32, Box<dyn Error>> {
    let request = format!(
        "https://malshare.com/api.php?api_key={}&action=getlimit",
        api_key
    );
    let tuple = reqwest::get(request).await?.text().await?;
    Ok(json::parse(&tuple)?[target]
        .as_str()
        .expect("cannot extract string")
        .parse()?)
}

/// GET allocated number of API key requests per day
pub async fn get_api_call_limit(api_key: &str) -> Result<u32, Box<dyn Error>> {
    _get_api_call_limit_helper(api_key, "LIMIT").await
}

/// GET allocated number of remaining API key requests per day
pub async fn get_remaining_api_calls(api_key: &str) -> Result<u32, Box<dyn Error>> {
    _get_api_call_limit_helper(api_key, "REMAINING").await
}

/// Download a sample by providing its hash
///
/// If no output path is provided, the sample will be put in the current directory with the
/// filename being equal to `<hash>.vir`.
/// If a path is provided, the file will be stored to that path.
pub async fn download(
    api_key: &str,
    hash: &str,
    output: Option<PathBuf>,
) -> Result<(), Box<dyn Error>> {
    let request = format!(
        "https://malshare.com/api.php?api_key={}&action=getfile&hash={}",
        api_key, hash
    );

    let path = if let Some(path) = output {
        if !path
            .parent()
            .expect("Unable to get parent of path")
            .exists()
        {
            println!("--- Path '{}' does not exist.", path.display());
            let new_path: PathBuf = format!("{}.vir", hash).into();
            println!("--- Writing to: {}", new_path.display());
            new_path
        } else {
            path
        }
    } else {
        format!("{}.vir", hash).into()
    };

    let sample = reqwest::get(request).await?.text().await?;

    let mut file = File::create(path)?;
    file.write_all(sample.as_bytes())?;
    Ok(())
}

/// List hashes from the last 24 hours in JSON format
pub async fn get_list(api_key: &str) -> Result<json::JsonValue, Box<dyn Error>> {
    let request = format!(
        "https://malshare.com/api.php?api_key={}&action=getlist",
        api_key
    );
    let result = reqwest::get(request).await?.text().await?;
    let json = json::parse(&result)?;
    Ok(json)
}

/// List hashes from the last 24 hours in raw format
pub async fn get_list_raw(api_key: &str) -> Result<String, Box<dyn Error>> {
    let request = format!(
        "https://malshare.com/api.php?api_key={}&action=getlistraw",
        api_key
    );
    let raw = reqwest::get(request).await?.text().await?;
    Ok(raw)
}

/// List the details of a file whose hash is provided in json format.
pub async fn list_details(api_key: &str, hash: &str) -> Result<json::JsonValue, Box<dyn Error>> {
    let request = format!(
        "https://malshare.com/api.php?api_key={}&action=details&hash={}",
        api_key, hash
    );
    let result = reqwest::get(request).await?.text().await?;
    let json = json::parse(&result)?;
    Ok(json)
}
