job "kafka" {
    datacenters = ["taboao"]
    group "kafka" {

        network {
            mode = "host"
            port "zookpeer" {
                to =  2181
                static = 2181
                
            }
            port "kafka" {
                to =  9092
                static = 9092
                
            }
        }

        task "kafka" {
            driver = "docker"
            config {
                image = "192.168.0.201:8099/library/kafka:latest"
                ports = ["kafka"]
            }
            env {
            
                KAFKA_ADVERTISED_HOST_NAME= "kafka"
                KAFKA_ZOOKEEPER_CONNECT= "192.168.0.201:2181"
            }
              resources {    
                cpu = 2000 
                memory=1000 
              }
        }
        task "zookpeer" {
            
            driver = "docker"
            lifecycle { 
                hook = "prestart" 
                sidecar = true 
            }
            config {
                image = "192.168.0.201:8099/library/zookeeper:latest"
                ports = ["zookpeer"]
            }
              resources {    
                cpu = 2000 
                memory=1000 
              }
        }
    }

}