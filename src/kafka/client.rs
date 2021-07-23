use rdkafka::ClientConfig;
use rdkafka::config::RDKafkaLogLevel;

pub fn default(host: &str , group_id: &str) -> ClientConfig {
    let mut client_config = ClientConfig::new();
    client_config.set("bootstrap.servers",host);
    client_config.set("group.id",group_id);
    client_config.set_log_level(RDKafkaLogLevel::Debug);
    client_config
}