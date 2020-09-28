use std::net::TcpListener;
#[actix_rt::test]
async fn health_check_works() {
    let address = spawn_app();

    let client = reqwest::Client::new();

    let response = client
        .get(&format!("{}/health", address.as_str()))
        .send()
        .await
        .expect("failed to execute request");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

#[actix_rt::test]
async fn subscribe_returns_200_for_valid_form_data() {
    let address = spawn_app();
    let client = reqwest::Client::new();

    let body = "name=le%20guin&email=ursula_le_gui%40gmail.com";

    let response = client
            .post(&format!("{}/subscriptions", address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(body)
            .send()
            .await
            .expect("failed to execute request");
    
    assert_eq!(200, response.status().as_u16());            
}

#[actix_rt::test]
async fn subscribe_returns_400_when_form_data_is_invalid() {
    let address = spawn_app();
    let client = reqwest::Client::new();

    let test_cases = vec![
        ("name=le%20guin", "missing the email"),
        ("email=ursula_le_guin%40gmail.com", "missing the name"),
        ("", "missing both name and email")
    ];

    for (invalid_body, error_message) in test_cases {
        let response = client
                .post(&format!("{}/subscriptions", address))
                .header("Content-Type", "application/x-www-form-urlencoded")
                .body(invalid_body)
                .send()
                .await
                .expect("failed to execute request");
                
                assert_eq!(
                    400, 
                    response.status().as_u16(),
                    "api didn't fail w/ 400 when payload was {}",
                    error_message
                );            
    }
}

fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("failed to bind random port");
    let port = listener.local_addr().unwrap().port();
    let server = zero2prod::run(listener).expect("failed to start the app");
    let _ = tokio::spawn(server);

    format!("http://127.0.0.1:{}", port)
}
