use rdkafka::config::{ClientConfig, RDKafkaLogLevel, FromClientConfig};
use rdkafka::producer::{BaseRecord, BaseProducer};
use rdkafka::consumer::{BaseConsumer, Consumer};
use rdkafka::admin::{AdminClient, NewTopic, TopicReplication};
use rdkafka::error::{RDKafkaErrorCode, KafkaError};



fn show_message(){

    let mut r:BaseConsumer = ClientConfig::new()
        .set("bootstrap.servers","localhost:9092")
        .set("group.id","rust_example_group_1")
        .set_log_level(RDKafkaLogLevel::Debug)
        .create()
        .unwrap();

     r.subscribe(&vec!["topic-test"]).unwrap();
    for c in r.iter() {
        println!("{:?}", c.unwrap());
    }
}


async fn create_topic(){
    let mut client_config = ClientConfig::new();
    client_config.set("bootstrap.servers","localhost:9092");
    client_config.set("group.id","rust_example_group_1");
    client_config.set_log_level(RDKafkaLogLevel::Debug);

    let topic_config = NewTopic::new("topic_top",10,TopicReplication::Fixed(1));

    let kfk_admin = AdminClient::from_config(&client_config).unwrap();

    let result_create_topic = kfk_admin.create_topics([topic_config].iter(), &Default::default()).await;

    match result_create_topic {
        Ok(d) => {
            println!("topic criado com sucesso {:#?}", d)
        }
        Err(d) => {
            println!("erro ao  criar o topic {:#?}", d)
        }
    }


}

fn send_message(){
    unimplemented!()
}

#[tokio::main]
async fn main() {
    
    create_topic().await;
}