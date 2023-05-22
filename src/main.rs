//! main.rs

use std::thread;
use std::time::Duration;
use futures_lite::stream::StreamExt;
use lapin::{
    options::*, publisher_confirm::Confirmation, types::FieldTable, BasicProperties, Connection,
    ConnectionProperties, Result,
};
// use serde_json::json;
use tracing::info;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

const CHANNEL_A:&'static str = "channel_a";

#[derive(Debug, Serialize, Deserialize)]
struct HelloMessage{
    id:Uuid,
    message:String,
}

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
        let queue = channel_a.queue_declare(CHANNEL_A, QueueDeclareOptions::default(), FieldTable::default()).await?;
        info!(?queue, "Declared queue");

        let mut consumer = channel_b
            .basic_consume(CHANNEL_A, "my_consumer", BasicConsumeOptions::default(), FieldTable::default())
            .await?;

        // consumer thread
        async_global_executor::spawn(async move {
            info!("consume");
            while let Some(message_result) = consumer.next().await {
                let message = message_result.expect("error in consumer");

                // info!("[consumer] message: {:?}", &message);

                // json deserialize
                // match serde_json::from_slice::<HelloMessage>(message.data.clone().as_slice()) {
                //     Ok(json)=>info!("[consumer] json: {:?}", &json),
                //     Err(e)=>info!("[consumer] json error {:?}", &e)
                // }

                // flexbuffer deserialize

                // let mut hello_message = message.data.clone().as_slice();

                match flexbuffers::from_slice::<HelloMessage>(message.data.clone().as_slice()){
                    Ok(hello_message)=>{
                        info!("[consumer] received (via flexbuffer): {:?}", &hello_message)
                    },
                    Err(e)=>info!("[consumer] flexbuffer deserialize error {:?}", &e)
                }
                message.ack(BasicAckOptions::default()).await.expect("ack");
            }
        }).detach();
        info!("spawned consumer; spawning publisher");

        // publish a message
        let mut s = flexbuffers::FlexbufferSerializer::new();


        // json -> serde
        // let payload = json!(HelloMessage{message:"Hello, world".to_string()}).to_string();
        // let payload = payload.as_bytes();

        loop {

            // flexbuffers -> serde
            let flex_hello = HelloMessage{ id:Uuid::new_v4(), message:"Hello, world".to_string()};
            info!("[publisher] sending: {:?}", &flex_hello);
            flex_hello.serialize(&mut s).unwrap();
            let payload = s.view();

            let confirmation = channel_a.basic_publish("", CHANNEL_A, BasicPublishOptions::default(), payload, BasicProperties::default()).await?.await?;
            // something about this block in the lapin example is lazy; adding this info! gets it to run
            info!("[publisher] confirmation: {:?}", &confirmation);
            assert_eq!(confirmation, Confirmation::NotRequested);
            thread::sleep(Duration::from_millis(100));
        }

    })

}