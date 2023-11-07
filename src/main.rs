use aws_sdk_s3::{Client, Error};


#[tokio::main]
async fn main() -> Result<(), Error> {
    let shared_config = aws_config::load_from_env().await;
    let client = Client::new(&shared_config);
    let req = client.list_buckets();
    let resp = req.send().await?;
    let buckets = resp.buckets.unwrap_or_default();
    for b in buckets.iter() {
        if let Some(name) = &b.name {
            println!("{}", name);
        }
    }
    Ok(())
}
