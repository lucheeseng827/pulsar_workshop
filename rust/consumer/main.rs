use pulsar::{
    client::{Client, Consumer, ConsumerOptions, Result},
    message::Message,
};

#[tokio::main]
async fn main() -> Result<()> {
    // Create a Pulsar client
    let client = Client::builder()
        .set_service_url("pulsar://localhost:6650")
        .build()
        .unwrap();

    // Create a consumer to consume data from the "my-topic" topic
    let mut consumer: Consumer = client
        .subscribe(
            ConsumerOptions::builder()
                .topic("persistent://public/default/my-topic")
                .subscription_name("my-subscription")
                .build(),
        )
        .await?;

    loop {
        // Receive a message from the topic
        let msg: Message = consumer.receive().await?;

        // Extract the data from the message
        let data = msg.data();

        // Consume the data from the database
        // ...

        // Acknowledge the message
        consumer.acknowledge(msg).await?;
    }
}
