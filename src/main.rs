mod constants;

use std::collections::HashSet;
use std::path::Path;

use constants::{BASEURL, IMGSAVEDIR};
use regex::Regex;
use reqwest::Result;
use tokio::fs::File;
use tokio::io::copy;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() -> Result<()> {
    loop {
        println!("--loop!--");

        let res = reqwest::get(BASEURL).await?;
        let body = res.text().await?;

        let re = match Regex::new(
            r"[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}_[0-9]*_[0-9a-z_]*.jpg",
        ) {
            Ok(r) => r,
            Err(_e) => break,
        };

        let matches: HashSet<&str> = re.find_iter(&body).map(|m| m.as_str()).collect();

        for filename in matches {
            let path = Path::new(IMGSAVEDIR).join(filename);

            if !path.exists() {
                println!("downloading file: {filename}");
                let url = format!("{BASEURL}{filename}");
                println!("url: {url}");
                let client = reqwest::Client::builder().build()?;
                let res = client.get(url).send().await?.bytes().await?;
                let mut data = res.as_ref();
                let mut file = File::create(path).await.unwrap();
                copy(&mut data, &mut file).await.unwrap();
                sleep(Duration::from_secs(1)).await;
            }
        }
        sleep(Duration::from_secs(60)).await;
    }

    Ok(())
}
