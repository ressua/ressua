use poem::{get, handler, listener::TcpListener, post, web::Json, web::Path, IntoResponse, Route, Server};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct SepaTransfer {
    sender_iban: String,
    receiver_iban: String,
    amount: f64,
    currency: String,
    reference: String,
}

#[handler]
fn hello(Path(name): Path<String>) -> String {
    format!("hello: {}", name)
}

#[handler]
async fn sepa_transfer(Json(transfer): Json<SepaTransfer>) -> impl IntoResponse {
    // Process the SEPA transfer here
    format!(
        "SEPA transfer from {} to {} of {} {} with reference: {}",
        transfer.sender_iban, transfer.receiver_iban, transfer.amount, transfer.currency, transfer.reference
    )
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let app = Route::new()
        .at("/hello/:name", get(hello))
        .at("/sepa_transfer", post(sepa_transfer));
    Server::new(TcpListener::bind("0.0.0.0:3000"))
        .run(app)
        .await
}

#[cfg(test)]
mod tests {
    use super::*;
    use reqwest;
    use tokio;

    async fn start_server(port: u16) {
        let app = Route::new()
            .at("/hello/:name", get(hello))
            .at("/sepa_transfer", post(sepa_transfer));
        Server::new(TcpListener::bind(format!("0.0.0.0:{}", port)))
            .run(app)
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn test_hello() {
        // Start the server in a background task on a different port
        let port = 3001;
        tokio::spawn(start_server(port));

        // Give the server a moment to start
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

        // Make a request to the server
        let client = reqwest::Client::new();
        let response = client
            .get(&format!("http://0.0.0.0:{}/hello/ressua-api", port))
            .send()
            .await
            .unwrap()
            .text()
            .await
            .unwrap();

        // Check the response
        assert_eq!(response, "hello: ressua-api");
    }

    #[tokio::test]
    async fn test_sepa_transfer() {
        // Start the server in a background task on a different port
        let port = 3002;
        tokio::spawn(start_server(port));

        // Give the server a moment to start
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

        // Make a POST request to the server
        let client = reqwest::Client::new();
        let transfer = SepaTransfer {
            sender_iban: "DE89370400440532013000".to_string(),
            receiver_iban: "FR7630006000011234567890189".to_string(),
            amount: 100.0,
            currency: "EUR".to_string(),
            reference: "Invoice 12345".to_string(),
        };
        let response = client
            .post(&format!("http://0.0.0.0:{}/sepa_transfer", port))
            .json(&transfer)
            .send()
            .await
            .unwrap()
            .text()
            .await
            .unwrap();

        // Check the response
        assert_eq!(
            response,
            "SEPA transfer from DE89370400440532013000 to FR7630006000011234567890189 of 100 EUR with reference: Invoice 12345"
        );
    }
}

// Test the server with curl
// curl http://localhost:3000/hello/ressua-api
/*
curl -X POST http://localhost:3000/sepa_transfer \
     -H "Content-Type: application/json" \
     -d '{
           "sender_iban": "DE89370400440532013000",
           "receiver_iban": "FR7630006000011234567890189",
           "amount": 100.0,
           "currency": "EUR",
           "reference": "Invoice 12345"
         }'
*/