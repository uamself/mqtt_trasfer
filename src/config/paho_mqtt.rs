use futures_lite::{future::block_on, StreamExt};
use paho_mqtt::AsyncClient;
use std::time::Duration;
use tokio::time::sleep;
extern crate paho_mqtt as mqtt;

// The topics to which we subscribe.
const TOPICS: &[&str] = &["$dp", "$dpl", "$dpc", "$dr", "$crep/pwd"];
const QOS: &[i32] = &[1; 5];

pub async fn mqtt_subscriber() {
    tokio::spawn(async move {
        do_subscribe().await;
    });
}

pub async fn get_client_of_local_mqtt() -> AsyncClient {
    let host = "mqtt://11.11.11.56:1883".to_string();
    let create_opts = mqtt::CreateOptionsBuilder::new()
        .server_uri(host)
        .client_id("60669947")
        .finalize();
    let cli: paho_mqtt::AsyncClient = mqtt::AsyncClient::new(create_opts).unwrap();
    cli
}

pub async fn do_subscribe() {
    let host = "mqtt://223.85.251.32:1883".to_string();

    let create_opts = mqtt::CreateOptionsBuilder::new()
        .server_uri(host)
        .client_id("14021115")
        .finalize();
    let mut cli: paho_mqtt::AsyncClient = mqtt::AsyncClient::new(create_opts).unwrap();

    if let Err(err) = block_on(async move {
        // Get message stream before connecting.
        let mut strm = cli.get_stream(25);

        // Define the set of options for the connection
        let lwt = mqtt::Message::new(
            "test/lwt",
            "[LWT] Async subscriber lost connection",
            mqtt::QOS_1,
        );

        let conn_opts = mqtt::ConnectOptionsBuilder::new_v3()
            .keep_alive_interval(std::time::Duration::from_secs(60))
            .user_name("n4vx8rc1ruyr")
            .password("ecxetzesxkremuwmpvvr")
            .clean_session(false)
            .will_message(lwt.clone())
            .finalize();

        // Make the connection to the broker
        let x = cli.connect(conn_opts.clone()).await;
        match x {
            Ok(_) => {
                println!("Connection established")
            }
            Err(e) => {
                println!("Error connecting to broker: {}", e)
            }
        }

        println!("Subscribing to topics: {:?}", TOPICS);
        cli.subscribe_many(TOPICS, QOS).await?;

        // Just loop on incoming messages.
        println!("Waiting for messages...");

        // -----------------------获取本地的连接start---------------------------
        // let client: paho_mqtt::AsyncClient = get_client_of_local_mqtt().await;
        // let conn_opts_local = mqtt::ConnectOptionsBuilder::new_v3()
        //     .keep_alive_interval(std::time::Duration::from_secs(20))
        //     .user_name("poyezkwcxw2b")
        //     .password("yc0amqq6i6oxy0b6j3f3")
        //     .clean_session(false)
        //     .will_message(lwt.clone())
        //     .finalize();
        // client.connect(conn_opts_local.clone()).await?;
        // -----------------------获取本地的连接end---------------------------

        while let Some(msg_opt) = strm.next().await {
            if let Some(msg) = msg_opt {
                let tp = msg.topic();
                println!("tp is {:?}", tp);
                // let msg = mqtt::Message::new(tp, msg.payload(), 1);
                // client.publish(msg);
            } else {
                // A "None" means we were disconnected. Try to reconnect...
                println!("Lost connection. Attempting reconnect...");
                while let Err(_) = cli.reconnect().await {
                    sleep(Duration::from_secs(1)).await;
                }
                println!("Reconnected.");
            }
        }
        Ok::<(), mqtt::Error>(())
    }) {
        eprintln!("{}", err);
    }
}
