#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct RawMqttEvent {
    pub battery: u8,
    pub humidity: f32,
    pub temperature: f32,
    pub voltage: u16,
    pub linkquality: Option<u8>,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct Event {
    pub device_name: String,
    pub battery: u8,
    pub humidity: f32,
    pub temperature: f32,
    pub voltage: u16,
    pub linkquality: Option<u8>,
}

impl Event {
    pub fn from_mqtt(source: RawMqttEvent, device_name: &str) -> Self {
        Self {
            device_name: device_name.to_string(),
            battery: source.battery,
            humidity: source.humidity,
            temperature: source.temperature,
            voltage: source.voltage,
            linkquality: source.linkquality,
        }
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct Subscription {
    pub topic: String,
    pub device_name: String,
}
