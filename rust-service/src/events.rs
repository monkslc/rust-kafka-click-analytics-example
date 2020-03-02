use serde::{Deserialize, Serialize};
use std::time::SystemTime;
use uuid::Uuid;

pub trait ToKafkaMessage {
    fn key(&self) -> Vec<u8>;
    fn payload(&self) -> Vec<u8>;
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Impression {
    pub data: String,
    pub id: Uuid,
    pub time: SystemTime,
    pub user_id: Uuid,
}

impl Impression {
    pub fn new(data: String, user_id: uuid::Uuid) -> Impression {
        Impression {
            data,
            id: Uuid::new_v4(),
            time: SystemTime::now(),
            user_id,
        }
    }

    pub fn from_bytes(payload: &[u8]) -> Result<Impression, &'static str> {
        match serde_cbor::from_slice(payload) {
            Ok(impression) => Ok(impression),
            Err(_) => Err("invalid payload"),
        }
    }
}

impl ToKafkaMessage for Impression {
    fn key(&self) -> Vec<u8> {
        self.id.as_bytes().to_vec()
    }

    fn payload(&self) -> Vec<u8> {
        serde_cbor::to_vec(self).expect("Can't serialize the event")
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Click {
    pub data: String,
    pub id: Uuid,
    pub impression_id: Uuid,
    pub time: SystemTime,
}

impl Click {
    pub fn new(data: String, impression_id: Uuid) -> Click {
        Click {
            data,
            id: Uuid::new_v4(),
            impression_id,
            time: SystemTime::now(),
        }
    }

    pub fn from_bytes(payload: &[u8]) -> Result<Click, &'static str> {
        match serde_cbor::from_slice(payload) {
            Ok(click) => Ok(click),
            Err(_) => Err("invalid payload"),
        }
    }
}

impl ToKafkaMessage for Click {
    fn key(&self) -> Vec<u8> {
        self.id.as_bytes().to_vec()
    }

    fn payload(&self) -> Vec<u8> {
        serde_cbor::to_vec(self).expect("Can't serialize the event")
    }
}
