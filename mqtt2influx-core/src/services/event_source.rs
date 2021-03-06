use crate::types::*;
use crate::AppError;
use anyhow::Result;
use rumqttc::{Event as MqttEvent, EventLoop, Incoming, MqttOptions, Publish, QoS, Request, Subscribe};
use tokio::sync::mpsc::{channel, Receiver, Sender};

#[async_trait::async_trait]
pub trait EventSource {
    async fn start(self) -> Result<Receiver<Event>>;
}

pub struct MqttEventSource {
    mqtt_options: MqttOptions,
    subscriptions: Vec<Subscription>,
}

pub struct MqttConnectionParameters<'a> {
    pub client_id: &'a str,
    pub host: &'a str,
    pub port: u16,
}

impl MqttEventSource {
    pub fn new(connection: MqttConnectionParameters, subscriptions: Vec<Subscription>) -> Self {
        let mut mqtt_options = MqttOptions::new(connection.client_id, connection.host, connection.port);
        mqtt_options.set_keep_alive(5);
        Self {
            mqtt_options,
            subscriptions,
        }
    }
}

#[async_trait::async_trait]
impl EventSource for MqttEventSource {
    async fn start(self) -> Result<Receiver<Event>> {
        let event_loop = EventLoop::new(self.mqtt_options, 10);
        let tx = event_loop.handle();
        for subscription in self.subscriptions.iter() {
            let topic = subscription.topic.clone();
            let subscribe = Subscribe::new(topic.clone(), QoS::AtMostOnce);
            let request = Request::Subscribe(subscribe);
            if let Err(e) = tx.send(request).await {
                return Err(AppError::Mqtt(format!("Error sending Subscribe request: {:?}", e)).into());
            } else {
                tracing::info!("Subscribed to [{}]", topic);
            }
        }

        let handler = SubscriptionHandler {
            subscriptions: self.subscriptions,
        };
        let (chan_tx, chan_rx) = channel::<Event>(10);
        tokio::spawn(async move {
            if let Err(e) = handler.run(event_loop, chan_tx).await {
                tracing::error!("Error in SubscriptionHandler: {}", e.to_string());
            }
        });

        Ok(chan_rx)
    }
}

struct SubscriptionHandler {
    subscriptions: Vec<Subscription>,
}

impl SubscriptionHandler {
    async fn run(&self, mut event_loop: EventLoop, tx: Sender<Event>) -> Result<()> {
        loop {
            match event_loop.poll().await {
                Ok(v) => {
                    if let MqttEvent::Incoming(Incoming::Publish(publish)) = v {
                        if let Err(e) = self.handle_publish(publish, &tx).await {
                            tracing::error!("Error handling publish: {}", e);
                        }
                    }
                }
                Err(e) => {
                    tracing::error!("MQTT connection error: {:?}", e);
                    break;
                }
            };
        }
        Ok(())
    }

    async fn handle_publish(&self, publish: Publish, tx: &Sender<Event>) -> Result<()> {
        let subscription = match self.subscriptions.iter().find(|s| s.topic == publish.topic) {
            Some(s) => s,
            None => {
                tracing::trace!("Received event for unknown subscription [topic={}]", publish.topic);
                return Ok(());
            }
        };

        let event: RawMqttEvent = serde_json::from_slice(&publish.payload)?;
        let converted = Event::from_mqtt(event, &subscription.device_name);
        tracing::trace!("Received event: {:?}", converted);
        tx.send(converted).await?;
        Ok(())
    }
}
