use reqwest::get;

pub async fn download_rom(url: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let response = get(url).await?;
    let data = response.bytes().await?;
    Ok(data.to_vec())
}
