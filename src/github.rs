use serde_derive::Deserialize;

#[derive(Deserialize, Debug)]
pub struct PushEvent {
    name: String,
}
