mod kafka;
use crate::kafka::client::default;
use crate::kafka::topic::create_topic;
use crate::kafka::message::{send, consumer, consumer_olds_messages};


use uuid::v1::{Context, Timestamp};


#[tokio::main]
async fn main() {


    let topic_name  = option_env!("TOPIC_NAME").unwrap();
    let host  = option_env!("HOST").unwrap();
    let group_id  = option_env!("GROUP_ID").unwrap();
    let mut client = default(host, group_id);

    create_topic(&client, 4,topic_name).await;



    let result = send(topic_name, uuid::Uuid::new_v4().to_string(), uuid::Uuid::new_v4().to_string(), &client).await;

    println!("{:#?}", result);

    tokio::join!(
        consumer(topic_name, default(host, group_id)),
        consumer_olds_messages(topic_name, default(host, group_id))
    );


}