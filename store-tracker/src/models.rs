#[allow(dead_code)]
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq)]
pub struct ProductDatabase {
    pub id: usize,
    pub title: String,
    pub variant_title: String,
    pub available: bool,
    pub price: String,
    pub from_db: bool,
}

#[derive(Debug, Clone)]
pub enum ProductWasChanged {
    PriceChange,
    NoLongerAvailable,
    IsAvailable,
    VariantTitle,
    ProductTitle,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductRespose {
    pub products: Vec<Product>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Product {
    pub id: usize,
    pub title: String,
    pub handle: String,
    pub body_html: String,
    pub published_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub vendor: String,
    pub product_type: String,
    pub tags: Vec<String>,
    pub variants: Vec<ProductVariant>,
    pub images: Vec<ProductImage>,
    pub options: Vec<ProductOption>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductVariant {
    pub id: usize,
    pub title: String,
    pub option1: Option<String>,
    pub option2: Option<String>,
    pub option3: Option<String>,
    pub sku: String,
    pub requires_shipping: bool,
    pub taxable: bool,
    pub featured_image: Option<ProductImage>,
    pub available: bool,
    pub price: String,
    pub grams: usize,
    pub compare_at_price: Option<String>,
    pub position: usize,
    pub product_id: usize,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductImage {
    pub id: usize,
    pub product_id: usize,
    pub position: usize,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub alt: Option<String>,
    pub width: usize,
    pub height: usize,
    pub src: String,
    pub variant_ids: Vec<usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductOption {
    pub name: String,
    pub position: usize,
    pub values: Vec<String>,
}
