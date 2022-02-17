use serde::{Deserialize};

#[derive(Deserialize, Debug, Clone, Default)]
pub struct Product {
  pub id: i64,
  pub handle: Option<String>,
  pub title: String,
  pub available: Option<bool>,
  pub variants: Option<Vec<Product>>
}

#[derive(Deserialize, Debug, Clone, Default)]
pub struct ShoppifyProducts {
  pub products: Option<Vec<Product>>
}
