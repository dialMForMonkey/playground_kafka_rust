
use kafka::client::FetchPartition;
use std::borrow::Borrow;
use std::ops::Deref;


fn main() {

//localhost 9092

    let mut client = kafka::client::KafkaClient::new(vec!("localhost:9092".to_owned()));
    client.load_metadata_all().unwrap();
    //list topic
    let topics: Vec<String> = client.topics().names().map(ToOwned::to_owned ).collect();
    let partition =  topics.iter().map(|s|{
        FetchPartition::new(s.as_str(),0,0)
    }).collect::<Vec<FetchPartition>>();
    //let a = &[FetchPartition::new()];
    let result = client.fetch_messages(&partition[..]).unwrap();
    //
    for m in result {
        for topic in m.topics() {
            for partition in topic.partitions() {
                match partition.data() {
                    Err(e)=>{
                        println!("error {:?}", e);
                    },Ok(d)=>{
                        for message in d.messages() {


                            println!("{:?}",   String::from_utf8_lossy(message.value));
                        }
                    }
                }
            }
        }
    }



        /****.map(|topic| {
            FetchPartition::new(&topic, 0, 0)
        })
        .collect::<Vec<FetchPartition>>();
    //client.fetch_messages(topics_name.deref()).unwrap();
    ****/





}
