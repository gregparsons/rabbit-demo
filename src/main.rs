//! main.rs

use futures_lite::stream::StreamExt;
use lapin::{
    options::*, publisher_confirm::Confirmation, types::FieldTable, BasicProperties, Connection,
    ConnectionProperties, Result,
};
use tracing::info;

fn main() -> Result<()> {
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info");
    }

    tracing_subscriber::fmt::init();

    // let addr = std::env::var("AMQP_ADDR").unwrap_or_else(|_| "amqp://127.0.0.1:5672/%2f".into());
    // let addr = std::env::var("AMQP_ADDR").unwrap_or_else(|_| "amqp://127.0.0.1:5672".into());
    let addr = std::env::var("AMQP_ADDR").unwrap_or_else(|_| "amqp://127.0.0.1:5672/my_vhost".into());

    async_global_executor::block_on(async {

        let conn = Connection::connect(&addr, ConnectionProperties::default()).await?;
        info!("CONNECTED");

        let channel_a = conn.create_channel().await?;
        let channel_b = conn.create_channel().await?;

        // establish queue named hello
        let queue = channel_a.queue_declare("hello", QueueDeclareOptions::default(), FieldTable::default()).await?;
        info!(?queue, "Declared queue");

        let mut consumer = channel_b
            .basic_consume("hello", "my_consumer", BasicConsumeOptions::default(), FieldTable::default())
            .await?;

        // consumer thread
        async_global_executor::spawn(async move {
            info!("will consume");
            while let Some(delivery) = consumer.next().await {
                let delivery = delivery.expect("error in consumer");
                delivery.ack(BasicAckOptions::default()).await.expect("ack");
            }
        }).detach();

        info!("spawned consumer; spawning publisher");

        // publish a message
        let payload = b"Hello world!";
        loop {
            let confirm = channel_a.basic_publish("", "hello", BasicPublishOptions::default(), payload, BasicProperties::default()).await?.await?;
            // something about this block in the lapin example is lazy; adding this info! gets it to run
            info!("[publish] confirmation: {:?}", &confirm);
            assert_eq!(confirm, Confirmation::NotRequested);
        }

    })

}