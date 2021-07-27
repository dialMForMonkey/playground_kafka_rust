use rdkafka::consumer::{StreamConsumer, Consumer};
use rdkafka::config::FromClientConfig;
use rdkafka::ClientConfig;
use rdkafka::producer::{FutureProducer, FutureRecord};
use rdkafka::message::{OwnedHeaders};
use std::time::Duration;
use futures::{StreamExt, TryStream};
use futures::stream::FuturesUnordered;
use rdkafka::util::{ Timeout};
use rdkafka::error::KafkaResult;


pub  async fn consumer(topic_name: &str, client_config: &ClientConfig) {

    let result_consumer:KafkaResult<StreamConsumer> =  client_config.create();

    match result_consumer {
        Ok(consumer)=> {
            consumer
                .subscribe(&[topic_name])
                .expect("erro no subscribe");

            consumer.stream().map(|result_msg|{
                println!("Only print message {:#?}", result_msg);
                None
            }).collect::<Vec<Option<i16>>>();
        },
        Err(d)=> {
            println!("Erro  client config {:#?}", d)

        }

    }
}


pub async fn send(topic_name :&str, key:String, message:String, client_config: &ClientConfig)-> Result<String,String>{

    let result_producer = FutureProducer::from_config(client_config);

    match result_producer {
        Ok(producer) => {

            let record_future =
                FutureRecord::to(topic_name)
                    .headers(OwnedHeaders::default())
                    .key(key.as_str())
                    .payload(key.as_str());

            let result = producer.send(record_future,Timeout::After(Duration::from_secs(30))).await;
            match result {
                Ok((partition_num, offset)) => {
                    Ok(format!("message saved in {:?} partition", partition_num))
                }
                Err((kfk_error,_)) => {
                    Ok(format!("error message save  {:#?} ", kfk_error))
                }
            }
        }
        Err(e) => {
            Err(format!("error ao salvar mensage: {:?}",e))
        }
    }
}

