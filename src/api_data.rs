use dioxus::prelude::*;

#[server]
async fn post_server_data(data: String) -> Result<(), ServerFnError> {
    println!("Server received: {}", data);

    Ok(())
}

#[server]
async fn get_server_data() -> Result<String, ServerFnError> {
    Ok(reqwest::get("https://httpbin.org/ip").await?.text().await?)
}
