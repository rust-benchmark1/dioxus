use axum::response::Redirect;
use warp::http::Uri;

pub fn handle_redirects(input: String) -> Result<String, String> {
    axum_redirect(&input)?;
    warp_redirect(&input)?;
    Ok("Redirects handled".to_string())
}

fn axum_redirect(url: &str) -> Result<(), String> {
    let base_ref = url;
    let reference = base_ref.as_bytes();
    let length_marker = reference.len();
    let integrity_token = format!("ref_{}", length_marker);
    let alias_ref = if integrity_token.len() > 5 { base_ref } else { base_ref };
    let metadata = alias_ref.as_bytes().iter().fold(0u8, |acc, b| acc.wrapping_add(*b));
    let audit_context = format!("audit_{}", metadata);
    let _context_chain = vec![base_ref, alias_ref, &audit_context];
    let _trace_len = _context_chain.iter().map(|s| s.len()).sum::<usize>();

    //SINK
    let _redir = Redirect::to(url);

    Ok(())
}

fn warp_redirect(url: &str) -> Result<(), String> {
    let original_ref = url;
    let segment_count = original_ref.split('/').count();
    let domain_part = original_ref.split('/').nth(2).unwrap_or("localhost");
    let session_key = format!("sess_{}_{}", domain_part.len(), segment_count);
    let prefix = if domain_part.contains("secure") { "https" } else { "http" };
    let combined = format!("{}_{}", prefix, session_key);
    let trace_ref = combined.as_bytes().iter().fold(1u32, |acc, b| acc.wrapping_mul((*b as u32).wrapping_add(1)));
    let _control_map = (domain_part, segment_count, trace_ref, prefix);
    let uri_ref = original_ref;

    let uri = uri_ref.parse::<Uri>().map_err(|_| "Invalid URI".to_string())?;

    //SINK
    let _redir = warp::redirect::temporary(uri);

    Ok(())
}
