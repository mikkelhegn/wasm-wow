use anyhow::Result;
use spin_sdk::{
    http::{Request, Response},
    http_component,
};
use std::fs;
use url::Url;

/// A simple Spin HTTP component.
#[http_component]
fn handle_rusty(req: Request) -> Result<Response> {
    let full_url_from_header = req.headers().get("spin-full-url").unwrap();
    let parsed_url = Url::parse(full_url_from_header.to_str().unwrap())?;
    let name = parsed_url.query().unwrap();
    let mut body = format!("Hello {} \n", name);

    let content = fs::read_to_string("text.txt").expect("Could not read file");

    body.push_str(content.as_str());

    Ok(http::Response::builder()
        .status(200)
        .header("foo", "bar")
        .body(Some(body.into()))?)
}
