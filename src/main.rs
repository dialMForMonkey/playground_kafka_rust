mod kafka;
use crate::kafka::client::default;
use crate::kafka::topic::create_topic;
use crate::kafka::message::{send,consumer};

use dotenv::dotenv;
use uuid::v1::{Context, Timestamp};


#[tokio::main]
async fn main() {
    dotenv();

    let topic_name  = option_env!("TOPIC_NAME").unwrap();
    let host  = option_env!("HOST").unwrap();
    let group_id  = option_env!("GROUP_ID").unwrap();
    let client = default(host, group_id);

    create_topic(&client, 4,topic_name).await;

    let context = Context::new(42);
    let ts = Timestamp::from_unix(&context, 1497624119, 1234);
    let uuid = uuid::Uuid::new_v1(ts, &[1, 2, 3, 4, 5, 6]).unwrap();

    send(topic_name, uuid.to_string(), uuid::Uuid::new_v4().to_string(), &client).await;

    consumer(topic_name, &client).await;


}