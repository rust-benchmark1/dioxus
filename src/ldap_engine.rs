use ldap3::LdapConn;
use simple_ldap::{LdapClient, LdapConfig};
use simple_ldap::ldap3::Scope as LdapScope;
use simple_ldap::filter::{EqFilter, ContainsFilter};
use url::Url;
use serde_json;

pub fn handle_ldap_operations(user_input: String) -> Result<String, String> {
    let original = user_input;

    let ctx_tag = prepare_context(&original);

    let _ = perform_simple_bind(&original, &ctx_tag)?;

    let rt = tokio::runtime::Runtime::new().map_err(|e| format!("Failed to create Tokio runtime: {}", e))?;
    rt.block_on(async { perform_ldap_search(&original, &ctx_tag).await })?;

    Ok("LDAP operations completed".to_string())
}

fn prepare_context(input: &str) -> String {
    let len_s = input.len().to_string();
    let mut marker = "ctx:".to_string();
    marker.push_str(&len_s);
    let mut token = "env_".to_string();
    token.push_str(&marker);
    token
}

fn perform_simple_bind(original: &str, ctx: &str) -> Result<(), String> {
    let host = "ldap://127.0.0.1:389";
    let mut conn_label = ctx.to_string();
    conn_label.push_str(":bind");

    let dn = original;

    let pw = if original.len() > 8 {
        original[..8].to_string()
    } else {
        "password".to_string()
    };

    let principal = dn.split(',').next().unwrap_or(dn);

    let mut bind_token = conn_label.clone();
    bind_token.push(':');
    bind_token.push_str(principal);
    let _ = bind_token.len();

    let mut ldap = LdapConn::new(host).map_err(|_| "Failed to connect to LDAP server".to_string())?;
    //SINK
    let _ = ldap.simple_bind(dn, &pw).map_err(|_| "LDAP simple_bind failed".to_string())?;

    Ok(())
}

async fn perform_ldap_search(filter_input: &str, ctx: &str) -> Result<(), String> {
    let cfg = LdapConfig {
        bind_dn: "cn=admin,ou=services,dc=example,dc=com".to_string(),
        bind_password: "ChangeMe!".to_string(),
        ldap_url: Url::parse("ldap://127.0.0.1:389").map_err(|_| "bad url".to_string())?,
        dn_attribute: None,
        connection_settings: None,
    };

    let mut client = LdapClient::new(cfg).await.map_err(|_| "client new failed".to_string())?;

    let base = "dc=example,dc=com";
    let filter = ContainsFilter::from("uid".to_string(), filter_input.to_string());
    let attrs = vec!["uid"];

    //SINK
    let _resp = client.search::<serde_json::Value>(base, LdapScope::Subtree, &filter, &attrs)
        .await
        .map_err(|_| "LDAP search failed".to_string())?;

    Ok(())
}