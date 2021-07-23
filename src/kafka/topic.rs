pub use rdkafka::ClientConfig;
use rdkafka::admin::{NewTopic, TopicReplication};

pub async fn create_topic(client_config :&ClientConfig, num_replication: i8, name: &str) -> Result<String, String>{


    let topic_config = NewTopic::new(
        name,1,TopicReplication::Fixed(num_replication as i32 ));

    let kfk_admin = AdminClient::from_config(&client_config).unwrap();

    let result_create_topic = kfk_admin.create_topics([topic_config].iter(), &Default::default()).await;

    match result_create_topic {
        Ok(d) => {
            Ok(format!("topic criado com sucesso {:#?}", d))
        }
        Err(d) => {
            Err(format!("erro ao  criar o topic {:#?}", d))
        }
    }
}
