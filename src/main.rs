use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use mqtt_trasfer::config::paho_mqtt::mqtt_subscriber;

// test
async fn test() -> impl Responder {
    HttpResponse::Ok().body("消息发送到MQTT")
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    mqtt_subscriber().await;

    println!("----------------数据转发程序启动成功----------------");

    // 启动服务
    HttpServer::new(move || App::new().route("/test", web::get().to(test)))
        .bind("0.0.0.0:8500")?
        .run()
        .await
}
