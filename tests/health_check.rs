// use testRUST::main;

use sqlx::{Connection, PgConnection};
use newsletter::configuratio::get_config;
use std::net::TcpListener;

#[tokio::test]
async fn test_health_check() {
    let addr =spawn_app();
    let client =reqwest::Client::new();
    let resp = client
        .get(&format!("{}/healthcheck",&addr))
        .send()
    .await.expect("Can't send request");
    spawn_app();
    assert!(resp.status().is_success());
    assert_eq!(Some(0), resp.content_length());

}
#[tokio::test]
async fn subscribe_returns_200(){
    let addr =spawn_app();
    let configuration = get_config().expect("Failed to read configuration");
    let connection_string = configuration.database.connection_string();
    let mut connection = PgConnection::connect(&connection_string)
        .await
        .expect("Failed to connect to Postgres.");
    let client = reqwest::Client::new();
    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";
    let resp = client
    .post(&format!("{}/Subscribe",&addr))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
    .send()
    .await.expect("Can't send request");
    assert_eq!(200,resp.status().as_u16());
    let saved = sqlx::query!("SELECT email , name FROM subscriptions",)
    .fetch_one(&mut connection)
    .await
    .expect("Failed to fetch saved subscription.");
    assert_eq!(saved.email, "ursula_le_guin@gmail.com");
    assert_eq!(saved.name, "le guin");
}
#[tokio::test]
async fn subscribe_returns_400(){
    let addr =spawn_app();
    let client = reqwest::Client::new();
    let body_case =vec![("name=le%20guin","missing email"),
                        ("email=ursula_le_guin%40gmail.com","missing name")];
    for (inv_body,errmsg) in body_case {
        let resp = client
            .post(&format!("{}/Subscribe",&addr))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(inv_body)
            .send()
            .await.expect("Can't send request");
        assert_eq!(400,resp.status().as_u16(),"payload was {}",errmsg);
    }

}
fn spawn_app() -> String {
   let lis=TcpListener::bind("127.0.0.1:0")
       .expect("Can't bind to 0");
   let port = lis.local_addr().unwrap().port();
    let server = newsletter::run(lis).expect("Can't run server");
    let _ = tokio::spawn(server);
    format!("http://127.0.0.1:{}", port)
}