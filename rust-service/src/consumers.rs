use futures::stream::StreamExt;
use rdkafka::{
    config::ClientConfig,
    consumer::{stream_consumer::StreamConsumer, Consumer},
    message::BorrowedMessage,
};

pub struct EventConsumer(pub StreamConsumer);

impl EventConsumer {
    pub fn new(topics: &[&str], group: &str) -> EventConsumer {
        let consumer: StreamConsumer = ClientConfig::new()
            .set("group.id", group)
            .set("bootstrap.servers", "kafka:9092")
            .create()
            .expect("couldn't create the consumer");

        consumer
            .subscribe(topics)
            .expect("Can't subscribe to the specified topics");

        EventConsumer(consumer)
    }

    pub fn clean_message_stream(
        &self,
    ) -> impl futures::stream::Stream<Item = BorrowedMessage> + '_ {
        let message_stream = self.0.start();
        return message_stream.filter_map(move |message| match message {
            Ok(message) => futures::future::ready(Some(message)),
            Err(err) => {
                println!("Error: {}", err);
                futures::future::ready(None)
            }
        });
    }
}
