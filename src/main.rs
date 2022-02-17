use product::ShoppifyProducts;
use tokio;
use reqwest::{self, header};
use anyhow::Result;
use once_cell::sync::Lazy;
use std::env;

mod product;

static USER_AGENT: &str = "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_9_3) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/35.0.1916.47 Safari/537.36";
static IFTTT_KEY: Lazy<String> = Lazy::new(|| {
  env::var("IFTTT_KEY").expect("IFTTT_KEY not set in environment")
});

async fn send_notification() -> Result<(), reqwest::Error> {
  return match reqwest::get(format!("https://maker.ifttt.com/trigger/dracula_available/json/with/key/{}", IFTTT_KEY.to_owned())).await {
    Ok(_) => Ok(()),
    Err(error) => Err(error)
  }
}

async fn scrape_target() -> Result<bool, reqwest::Error> {
  let response = reqwest::Client::new()
    .get("https://oblotzky.industries/products.json")
    .header(header::USER_AGENT, USER_AGENT)
    .send()
    .await?;

  let products: ShoppifyProducts = response.json::<ShoppifyProducts>().await?;
  if let Some(products) = products.products {
    println!("Yay got {} products!", products.len());
    if let Some(dracula_product) = products.into_iter().find(|x| {
      if let Some(handle) = &x.handle {
        return handle == "gmk-dracula-v2"
      } else { return false }
    }) {
      println!("Nice found the dracula v2 set");
      if let Some(variants) = dracula_product.variants {
        println!("Yes! {} Variants available", variants.len());
        if let Some(core) = variants.into_iter().find(|x| &x.title ==  "Main Core") {
          return match core.available {
            Some(val) => Ok(val),
            None => Ok(false)
          }
        }
      }
    };
  } else {
    return Ok(false);
  }
  
  Ok(true)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  
  let is_available = scrape_target().await?;
  if is_available {
    send_notification().await?
  }

  if is_available {
    println!("Shit yea dracula v2 available! GO GET IT!!!");
  } else {
    println!("Dang, not yet available");
  }

  Ok(())
}
