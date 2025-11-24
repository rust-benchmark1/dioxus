use md5;
use md2::Md2;
use digest::Digest;
use hex;

pub fn compute_md5(input: &str, ctx: &str) -> Result<String, String> {
    let mut combined = String::new();
    combined.push_str(ctx);
    combined.push(':');
    combined.push_str(input);
    let salt = combined.len() as u8;
    let mut payload = Vec::with_capacity(input.len() + 1);
    payload.extend_from_slice(input.as_bytes());
    payload.push(salt);
    //SINK
    let digest = md5::compute(&payload);
    let hexed = format!("{:x}", digest);
    Ok(hexed)
}

pub fn compute_md2(input: &str, ctx: &str) -> Result<String, String> {
    let mut ctx_buf = Vec::new();
    ctx_buf.extend_from_slice(ctx.as_bytes());
    ctx_buf.push(b'#');
    let mut rolling = 0u32;
    for b in input.bytes().take(32) {
        rolling = rolling.wrapping_add(b as u32);
    }
    ctx_buf.extend_from_slice(&rolling.to_le_bytes());
    ctx_buf.extend_from_slice(input.as_bytes());
    //SINK
    let result = md2::Md2::digest(&ctx_buf);
    let hexed = hex::encode(result);
    Ok(hexed)
}

