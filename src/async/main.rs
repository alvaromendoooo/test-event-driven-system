use chrono::Utc;
use event_driven_system::events::OrderEvent;
use tokio::sync::broadcast;
use uuid::Uuid;

#[tokio::main]
async fn main(){
    // Creates a broadcast channel with buffer capacity of 100 events
    let (tx, _rx) = broadcast::channel::<OrderEvent>(100);

    // Subscribe multiple receivers to the same channel
    let mut rx1 = tx.subscribe();
    let mut rx2 = tx.subscribe();

    // Spawn the first consumer task - handles logging
    let _logger = tokio::spawn(async move {
        while let Ok(event) = rx1.recv().await {
            println!("[LOGGER] Event received: {:?}", event);
        }
    });

    // Spawn the first consumer task - handles login
    let _analytics = tokio::spawn(async move {
        while let Ok(event) = rx2.recv().await {
            if let OrderEvent::PaymentReceived { amount, ..} = event {
                println!("[ANALYTICS] Revenue tracked: ${}", amount);
            }
        }
    });

    // Send an event that both consumers will receive
    let event = OrderEvent::PaymentReceived { 
        order_id: Uuid::new_v4(), 
        amount: 99.99, 
        payment_method: "paypal".to_string(), 
        timestamp: Utc::now() 
    };
    tx.send(event).expect("Failed to broadcast the event");

    // Give consumers time to process
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
}