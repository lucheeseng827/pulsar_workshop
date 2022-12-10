use kafka::producer::{Producer, Record, RequiredAcks};

let mut producer = Producer::from_hosts(vec!["kafka:9092".to_string()])
    .with_ack_timeout(Duration::from_secs(10))
    .with_required_acks(RequiredAcks::One)
    .create()
    .unwrap();

let topic = "foos";
let partition = 0;
let key = b"key";
let payload = encoded;

let record = Record::from_key_value(key, payload);
producer
    .send(&Record::from_key_value(key, payload), topic, partition)
    .unwrap();



    // use kafka::consumer::{Consumer, FetchOffset, GroupOffsetStorage
