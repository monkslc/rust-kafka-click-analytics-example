use crate::events::ToKafkaMessage;
use rdkafka::{
    config::ClientConfig,
    producer::{FutureProducer, FutureRecord}
};

pub struct Producer(FutureProducer);

impl Producer {
    pub fn new() -> Producer {
        let producer: FutureProducer = ClientConfig::new()
            .set("bootstrap.servers", "kafka:9092")
            .set("message.timeout.ms", "5000")
            .create()
            .expect("Can't create the producer");

        Producer(producer)
    }

    pub async fn send_event<T: ToKafkaMessage>(&self, event: &T, topic: &str) -> Result<(), &'static str> {
        let payload = event.payload();
        let key = event.key();
        let message = FutureRecord::to(topic).payload(&payload).key(&key);
        let delivered_message = self.0.send(message, 0).await;

        match delivered_message {
            Ok(Ok(_)) => {
                println!("Successfully delivered event");
                Ok(())
            }
            Ok(Err(_)) => {
                println!("Error delivering message");
                Err("Error Delivering the message")
            }
            Err(e) => {
                println!("Error delivering message: {}", e);
                Err("Hey, error delivering the message")
            }
        }
    }
}
