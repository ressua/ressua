use poem::{get, handler, listener::TcpListener, web::Path, IntoResponse, Route, Server};

#[handler]
fn hello(Path(name): Path<String>) -> String {
    format!("hello: {}", name)
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let app = Route::new().at("/hello/:name", get(hello));
    Server::new(TcpListener::bind("0.0.0.0:3000"))
        .run(app)
        .await
}

#[cfg(test)]
mod tests {
    use super::*;
    use reqwest;
    use tokio;

    #[tokio::test]
    async fn test_hello() {
        // Start the server in a background task
        tokio::spawn(async {
            let app = Route::new().at("/hello/:name", get(hello));
            Server::new(TcpListener::bind("0.0.0.0:3000"))
                .run(app)
                .await
                .unwrap();
        });

        // Give the server a moment to start
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

        // Make a request to the server
        let client = reqwest::Client::new();
        let response = client
            .get("http://0.0.0.0:3000/hello/ressua-api")
            .send()
            .await
            .unwrap()
            .text()
            .await
            .unwrap();

        // Check the response
        assert_eq!(response, "hello: ressua-api");
    }
}