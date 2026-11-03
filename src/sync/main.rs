use std::sync::mpsc;
use std::thread;
use chrono::Utc;
use event_driven_system::events::{OrderEvent, OrderItem};
use uuid::Uuid;

fn main() {
    // Creating channels for sender and receiver
    let (sendx, recx) = mpsc::channel::<OrderEvent>();

    // Cloning sender for multiple producers
    let send2x = sendx.clone();

    // First producer thread - creates an order
    let producer1 = thread::spawn(move || {
        let event = OrderEvent::Created { 
            order_id: Uuid::new_v4(), 
            customer_id: Uuid::new_v4(), 
            items: vec![OrderItem {
                product_id: Uuid::new_v4(),
                quantity: 2,
                price: 29.99
            }], 
            timestamp: Utc::now() 
        };
        sendx.send(event).expect("Failed to send the event");
    });

    // Second producer thread - record a payment
    let producer2 = thread::spawn(move || {
        let event = OrderEvent::PaymentReceived { 
            order_id: Uuid::new_v4(), 
            amount: 59.98, 
            payment_method: "credit_card".to_string(), 
            timestamp: Utc::now() 
        };
        send2x.send(event).expect("Failed to send the event");
    });

    // Consumer processes all incoming events
    for event in recx {
        match event {
            OrderEvent::Created { order_id, .. } => {
                println!("Processing new order: {}", order_id);
            },
            OrderEvent::PaymentReceived { order_id, amount, .. } => {
                println!("Payment of ${} received for order {}", amount, order_id);
            },
            _ => {},
        }
    }

    // Wait for all processes to finish
    producer1.join().unwrap();
    producer2.join().unwrap();
}
