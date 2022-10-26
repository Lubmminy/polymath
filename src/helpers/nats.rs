use once_cell::sync::OnceCell;
static CLIENT: OnceCell<nats::Connection> = OnceCell::new();

pub async fn init() -> std::io::Result<nats::Subscription> {
    let client = nats::connect("localhost")?;
    let sub = client.subscribe("polymath")?;

    let _ = CLIENT.set(client);

    Ok(sub)
}

pub fn publish(message: String) -> std::io::Result<()> {
    CLIENT.get().unwrap().publish("polymath", message)?;
    
    Ok(())
}