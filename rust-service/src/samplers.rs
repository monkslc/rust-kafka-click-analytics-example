use rand::prelude::*;
use uuid::Uuid;

use crate::events;

pub struct ImpressionSampler {
    pub user_id: Uuid,
}

impl ImpressionSampler {
    pub fn new(user_id: Uuid) -> ImpressionSampler {
        ImpressionSampler { user_id }
    }
}

impl Iterator for ImpressionSampler {
    type Item = events::Impression;

    fn next(&mut self) -> Option<Self::Item> {
        Some(events::Impression::new(
            String::from("Random impression data"),
            self.user_id,
        ))
    }
}

pub struct ClickSampler {
    click_probability: f64,
    impression_id: Uuid,
}

impl ClickSampler {
    pub fn new(impression_id: Uuid, click_probability: f64) -> ClickSampler {
        ClickSampler {
            click_probability,
            impression_id,
        }
    }
}

impl Iterator for ClickSampler {
    type Item = events::Click;

    fn next(&mut self) -> Option<Self::Item> {
        let rndm: f64 = random();
        if rndm < self.click_probability {
            Some(events::Click::new(
                String::from("Click Data"),
                self.impression_id,
            ))
        } else {
            None
        }
    }
}
