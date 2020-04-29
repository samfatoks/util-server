use crate::error::Error;

pub fn base64_encode(data: &str) -> Result<String, std::io::Error> {
    Ok(base64::encode(data))
}

pub fn base64_decode(data: &str) -> Result<String, Error> {
    let bytes = base64::decode(data)?;
    Ok(std::str::from_utf8(&bytes)?.to_string())
}
