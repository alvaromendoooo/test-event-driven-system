use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OrderEvent {
    Created {
        order_id: Uuid,
        customer_id: Uuid,
        items: Vec<OrderItem>,
        timestamp: DateTime<Utc>,
    },
    PaymentReceived {
        order_id: Uuid,
        amount: f64,
        payment_method: String,
        timestamp: DateTime<Utc>,
    },
    Shipped {
        order_id: Uuid,
        tracking_number: String,
        carrier: String,
        timestamp: DateTime<Utc>,
    },
    Delivered {
        order_id: Uuid,
        timestamp: DateTime<Utc>,
    },
    Cancelled {
        order_id: Uuid,
        reason: String,
        timestamp: DateTime<Utc>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderItem {
    pub product_id: Uuid,
    pub quantity: u32,
    pub price: f64,
}