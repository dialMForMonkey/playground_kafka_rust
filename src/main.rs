use rdkafka::config::{ClientConfig, RDKafkaLogLevel, FromClientConfig};
use rdkafka::producer::{BaseRecord, BaseProducer};
use rdkafka::consumer::{BaseConsumer, Consumer};
use rdkafka::admin::{AdminClient, NewTopic, TopicReplication};
use rdkafka::error::{RDKafkaErrorCode, KafkaError};
use rdkafka::message::{OwnedHeaders, BorrowedMessage};
use std::time::Duration;
use rdkafka::Message;


fn get_client_config() -> ClientConfig{
    let mut client_config = ClientConfig::new();
    client_config.set("bootstrap.servers","localhost:9092");
    client_config.set("group.id","rust_example_group_1");
    client_config.set_log_level(RDKafkaLogLevel::Debug);
    client_config
}

fn show_message(){

    let r:BaseConsumer = get_client_config()
        .create()
        .unwrap();

     r.subscribe(&vec!["topic_top"]).unwrap();

    loop {
        let opt_message = r.poll(None);
        match opt_message {
            None => {
                println!("acabaram as mensagens");
                break
            }
            Some(resultkfk) => {
                println!("{:?}", String::from_utf8_lossy(resultkfk.unwrap().payload().unwrap() ) );
            }
        }
    }
}


async fn create_topic(){
    let  client_config = get_client_config();

    let topic_config = NewTopic::new("topic_top",1,TopicReplication::Fixed(1));

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
    let producer:BaseProducer = get_client_config().create().unwrap();


    let record = BaseRecord::to("topic_top").payload("t").key("a").partition(1).headers(OwnedHeaders::default());

    match producer.send(record) {
        Ok(_) => {
            println!("enviado com sucesso")
        }
        Err(e) => {
            println!("{:#?}", e)
        }
    }

    producer.poll(Duration::from_secs(1));


}

#[tokio::main]
async fn main() {

    send_message();
    send_message();
    send_message();
    show_message();
    create_topic().await;


}