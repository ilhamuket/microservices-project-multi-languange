use lapin::{
    options::*, types::FieldTable, Connection,
    ConnectionProperties, Consumer,
};
use crate::config::Config;
use crate::models::{NewStudent, UserRegisteredEvent};
use crate::db;
use sqlx::{Pool, Postgres};
use log::{info, error};
use futures::StreamExt;
use std::error::Error as StdError;

pub async fn setup_consumer(
    config: &Config,
    db_pool: Pool<Postgres>,
) -> std::result::Result<(), Box<dyn StdError>> {
    let conn = Connection::connect(
        &config.amqp_url(),
        ConnectionProperties::default(),
    )
    .await?;

    let channel = conn.create_channel().await?;

    // Declare exchange
    channel
        .exchange_declare(
            "user_events",
            lapin::ExchangeKind::Topic,
            ExchangeDeclareOptions {
                durable: true,
                ..ExchangeDeclareOptions::default()
            },
            FieldTable::default(),
        )
        .await?;

    // Declare queue
    let _queue = channel
        .queue_declare(
            "student_service_queue",
            QueueDeclareOptions {
                durable: true,
                ..QueueDeclareOptions::default()
            },
            FieldTable::default(),
        )
        .await?;

    // Bind queue to exchange
    channel
        .queue_bind(
            "student_service_queue",
            "user_events",
            "user.registered",
            QueueBindOptions::default(),
            FieldTable::default(),
        )
        .await?;

    // Start consuming messages
    let consumer = channel
        .basic_consume(
            "student_service_queue",
            "student_consumer",
            BasicConsumeOptions::default(),
            FieldTable::default(),
        )
        .await?;

    // Process messages
    tokio::spawn(async move {
        process_messages(consumer, db_pool).await;
    });

    Ok(())
}

async fn process_messages(mut consumer: Consumer, db_pool: Pool<Postgres>) {
    while let Some(delivery) = consumer.next().await {
        match delivery {
            Ok(delivery) => {
                let data = String::from_utf8_lossy(&delivery.data);
                info!("Received message: {}", data);

                match serde_json::from_str::<UserRegisteredEvent>(&data) {
                    Ok(event) => {
                        info!("Processing user registered event: {:?}", event);
                        
                        // Create student record
                        let new_student = NewStudent {
                            user_id: event.data.id,
                            name: event.data.name,
                            email: event.data.email,
                            phone_number: event.data.phone_number,
                            student_id: None, // Will be generated later
                        };

                        // Insert student into database
                        match db::create_student(&db_pool, &new_student).await {
                            Ok(_) => info!("Student created successfully"),
                            Err(e) => error!("Failed to create student: {}", e),
                        }
                    },
                    Err(e) => error!("Failed to parse event: {}", e),
                }

                // Acknowledge the message
                if let Err(e) = delivery.ack(BasicAckOptions::default()).await {
                    error!("Failed to acknowledge message: {}", e);
                }
            },
            Err(e) => error!("Failed to consume message: {}", e),
        }
    }
}