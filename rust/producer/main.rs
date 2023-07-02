use pulsar::{client::{Client, Producer, Result}, message::ProducerMessage};
use rusoto_s3::{
    GetObjectRequest, PutObjectRequest, S3Client, S3,
};

#[tokio::main]
async fn main() -> Result<()> {
    // Create a Pulsar client
    let client = Client::builder()
        .set_service_url("pulsar://localhost:6650")
        .build()
        .unwrap();

    // Create a producer to produce data to the "my-topic" topic
    let producer: Producer = client
        .create_producer(("persistent://public/default/my-topic", None))
        .await?;

    // Create an S3 client
    let s3_client = S3Client::new(Region::UsEast1);

    // Read the data from the file on S3
    let object = s3_client
        .get_object(GetObjectRequest {
            bucket: "my-bucket".to_owned(),
            key: "my-file.txt".to_owned(),
            ..Default::default()
        })
        .await?;

    let data = object.body.unwrap();

    // Produce the data to the Pulsar topic
    let _ = producer
        .send(ProducerMessage::new(data))
        .await?;
    }
