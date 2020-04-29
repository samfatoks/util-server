use ring::digest::{Context, SHA1_FOR_LEGACY_USE_ONLY, SHA256, SHA512};
use std::io::Read;

pub fn sha1_digest(data: &str) -> Result<String, std::io::Error> {
    let context = Context::new(&SHA1_FOR_LEGACY_USE_ONLY);
    sha_digest(data.as_bytes(), context)
}

pub fn sha256_digest(data: &str) -> Result<String, std::io::Error> {
    let context = Context::new(&SHA256);
    sha_digest(data.as_bytes(), context)
}

pub fn sha512_digest(data: &str) -> Result<String, std::io::Error> {
    let context = Context::new(&SHA512);
    sha_digest(data.as_bytes(), context)
}

fn sha_digest<R: Read>(mut reader: R, mut context: Context) -> Result<String, std::io::Error> {
    let mut buffer = [0; 1024];
    loop {
        let count = reader.read(&mut buffer)?;
        if count == 0 {
            break;
        }
        context.update(&buffer[..count]);
    }

    Ok(hex::encode(context.finish()))
}
