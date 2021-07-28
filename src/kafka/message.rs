use rdkafka::consumer::{StreamConsumer, Consumer};
use rdkafka::config::FromClientConfig;
use rdkafka::{ClientConfig, Message};
use rdkafka::producer::{FutureProducer, FutureRecord};
use rdkafka::message::{OwnedHeaders, BorrowedMessage};
use std::time::Duration;
use futures::{StreamExt, TryStream};
use futures::stream::FuturesUnordered;
use rdkafka::util::{ Timeout};
use rdkafka::error::{KafkaResult, KafkaError};


pub  async fn consumer_olds_messages(topic_name: &str, mut client_config: ClientConfig) {

    let result_consumer:KafkaResult<StreamConsumer> =  client_config
        .set("auto.offset.reset","earliest" )
        .set("group.id","read_all_msg_4" )
        .create();

    match result_consumer {
        Ok(consumer)=> {
            consumer
                .subscribe(&[topic_name])
                .expect("erro no subscribe");

            loop {
                let a = consumer.recv().await ;

                match a {
                    Ok(message) => {
                            let empty = "empty".as_bytes();
                            println!("message key {:?} message value {:?}",
                                     String::from_utf8_lossy(message.key().unwrap_or(empty)),
                                     String::from_utf8_lossy(message.payload().unwrap_or(empty))
                            );
                    }
                    Err(e) => {
                        println!("sem mensagem {:?}",e);
                        break
                    }
                }
            }


            consumer.stream()
                .map(|result_msg|{
                    println!("Only print message {:#?}", result_msg);
                    None
                })
                .collect::<Vec<Option<i16>>>()
                .await;
        },
        Err(d)=> {
            println!("Erro  client config {:#?}", d)

        }

    }
}



pub  async fn consumer(topic_name: &str, client_config: ClientConfig) {

    let result_consumer:KafkaResult<StreamConsumer> =  client_config
        .create();

    match result_consumer {
        Ok(consumer)=> {
            consumer
                .subscribe(&[topic_name])
                .expect("erro no subscribe");
            consumer.stream()
                .map(|result_msg|{

                    match result_msg  {
                        Ok(msg) => {
                            let empty = "empty".as_bytes();
                            println!("message key {:?} message value {:?}",
                                     String::from_utf8_lossy(msg.key().unwrap_or(empty)),
                                     String::from_utf8_lossy(msg.payload().unwrap_or(empty))
                            );
                        }
                        Err(e) => {
                            println!("error receive message {:?}", e);
                        }
                    }



                    None
                })
                .collect::<Vec<Option<i16>>>()
                .await;
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
                    Err(format!("error message save  {:#?} ", kfk_error))
                }
            }
        }
        Err(e) => {
            Err(format!("error ao salvar mensage: {:?}",e))
        }
    }
}

