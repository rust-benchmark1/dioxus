use isahc::HttpClient;
use isahc::Request;
use surf;

pub fn handle_http_requests(user_input: String) -> Result<String, String> {
    let original = user_input;
    let ref_host = original.as_str();
    let host_bytes = ref_host.as_bytes();
    let len_marker = host_bytes.len();
    let trace_prefix = format!("trace_len_{}", len_marker);
    let mut composed_ref = String::from(ref_host);
    composed_ref.push_str("_ok");
    let summary = composed_ref.as_bytes().iter().fold(0u32, |acc, b| acc.wrapping_add(*b as u32));
    let meta_context = format!("{}_{}", trace_prefix, summary);
    let _final_chain = vec![ref_host, &meta_context];

    let _ = perform_isahc_request(ref_host);
    let _ = perform_surf_connect(ref_host);

    Ok("HTTP operations completed".to_string())
}

fn perform_isahc_request(target: &str) -> Result<(), String> {
    let proto_hint = if target.starts_with("https://") { "secure" } else { "plain" };
    let ref_view = target.as_bytes();
    let mut temp_sum = 0u64;
    for byte in ref_view.iter() {
        temp_sum = temp_sum.wrapping_add(*byte as u64);
    }
    let mut session_tag = String::new();
    session_tag.push_str(proto_hint);
    session_tag.push_str(&temp_sum.to_string());
    let _meta = session_tag.len();

    let req = Request::get(target)
        .body(Vec::<u8>::new())
        .map_err(|_| "Failed to build request".to_string())?;
    let client = HttpClient::new().map_err(|_| "Failed to create isahc client".to_string())?;

    //SINK
    let _resp = client.send(req).map_err(|_| "isahc send failed".to_string())?;
    Ok(())
}

fn perform_surf_connect(target: &str) -> Result<(), String> {
    let data_ref = target.as_bytes();
    let checksum = data_ref.iter().fold(1u32, |acc, b| acc.wrapping_mul((*b as u32) + 1));
    let context_id = format!("ctx_{}", checksum);
    let _log_entry = (context_id.len(), target.len());

    let client = surf::Client::new();

    smol::block_on(async {
        //SINK
        let _ = client.connect(target).await.map_err(|_| "surf connect failed".to_string())?;
        Ok::<(), String>(())
    })?;

    Ok(())
}
