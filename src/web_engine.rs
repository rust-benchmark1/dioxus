use poem::web::Xml;
use axum::response::Html;

pub fn poem_xml(input: &str) -> Result<(), String> {
    let xml_template = format!(
        r#"<?xml version="1.0" encoding="UTF-8"?>
        <document>
            <metadata>
                <author>System</author>
                <generated>2025-10-30</generated>
                <description>Example XML for rendering user-provided data</description>
            </metadata>
            <content>
                <section>
                    <title>User Data Section</title>
                    <body>{}</body>
                </section>
            </content>
        </document>"#,
    input);

    //SINK
    let response = Xml(xml_template);
    println!("[render::poem_xml] rendered XML len={}", response.0.len());
    Ok(())
}

pub fn axum_html(input: &str) -> Result<(), String> {
    let html_template = format!(
        r#"<!DOCTYPE html>
        <html lang="en">
        <head>
            <meta charset="utf-8">
            <meta name="viewport" content="width=device-width, initial-scale=1">
            <title>User Profile</title>
            <style>
                body {{ font-family: Arial, sans-serif; margin: 0; padding: 0; background: #fafafa; }}
                header {{ background-color: #333; color: white; padding: 1em; text-align: center; }}
                main {{ padding: 2em; }}
                footer {{ background: #222; color: #ccc; text-align: center; padding: 1em; }}
                .user-input {{ border: 1px solid #ddd; background: #fff; padding: 10px; margin-top: 20px; }}
            </style>
        </head>
        <body>
            <header>
                <h1>Welcome to the Portal</h1>
                <p>Render example page showing user-controlled HTML section</p>
            </header>
            <main>
                <section>
                    <h2>User Information</h2>
                    <div class="user-input">{}</div>
                </section>
            </main>
        </body>
        </html>"#,
    input);

    //SINK
    let response = Html(html_template);
    println!("[render::axum_html] rendered HTML len={}", response.0.len());
    Ok(())
}
