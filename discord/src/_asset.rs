use std::str::Bytes;

#[derive(Debug, Clone)]
pub struct Asset<'a> {
    pub key: &'a str,
    pub url: &'a str,
}

impl <'a>Asset<'a> {
    pub fn is_animated() -> bool {

    }
    pub async fn read() -> Bytes {

    }
    pub fn replace(size: &'a u8, format: &str, static_format: &str) -> Self {

    }
}