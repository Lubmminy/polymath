mod helpers;

#[tokio::main]
async fn main() {
    helpers::get::get("test".to_string()).await;
}
