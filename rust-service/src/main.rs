use chrono::Local;
use futures::stream::StreamExt;
use rdkafka::{
    consumer::{CommitMode, Consumer},
    message::Message,
};
use std::time::Duration;
use tokio::time;
use tokio;
use uuid::Uuid;

use consumers::EventConsumer;
use events::{Click, Impression};
use producers::Producer;
use samplers::{ClickSampler, ImpressionSampler};

mod consumers;
mod events;
mod producers;
mod samplers;

#[tokio::main]
async fn main() {
    println!("{} : Starting up...", Local::now());

    let db = sled::open(".data").expect("can't connect to db");
    let producer = Producer::new();
    futures::join!(
        generate_impression_data(&producer),
        generate_impression_data(&producer),
        generate_impression_data(&producer),
        save_impressions(&db),
        save_clicks(&db)
    );
}

async fn generate_impression_data(producer: &Producer) {
    let impression_sampler = ImpressionSampler::new(Uuid::new_v4());
    for impression in impression_sampler {
        if let Ok(_) = producer.send_event(&impression, "impressions").await {
            for click in ClickSampler::new(impression.id, 0.10) {
                if let Err(e) = producer.send_event(&click, "clicks").await {
                    println!("Error sending event: {}", e);
                }
            }
        }
        time::delay_for(Duration::from_secs(3)).await;
    }
}

pub async fn save_impressions(db: &sled::Db) {
    let consumer = EventConsumer::new(&["impressions"], "save_impression");
    let mut message_stream = consumer.clean_message_stream();

    while let Some(message) = message_stream.next().await {
        if let Some(payload) = message.payload() {
            match Impression::from_bytes(payload) {
                Ok(impression) => {
                    println!("Handling Impression: {}", impression.id);
                    db.insert(impression.id.as_bytes(), payload).expect("Failed insert");
                }
                Err(e) => println!("Invalid impression payload: {}", e)
            }
            if let Err(e) = consumer.0.commit_message(&message, CommitMode::Async) {
                println!("Error commiting: {}", e);
            }
        }
    }
}

pub async fn save_clicks(db: &sled::Db) {
    let consumer = EventConsumer::new(&["clicks"], "save_click");
    let mut message_stream = consumer.clean_message_stream();

    while let Some(message) = message_stream.next().await {
        if let Some(payload) = message.payload() {
            match Click::from_bytes(payload) {
                Ok(click) => {
                    println!("Handling Click: {}", click.id);
                    db.insert(click.id.as_bytes(), payload).expect("Failed insert");
                }
                Err(e) => println!("Invalid click payload: {}", e)
            }
            if let Err(e) = consumer.0.commit_message(&message, CommitMode::Async) {
                println!("Error commiting: {}", e);
            }
        }
    }
}
