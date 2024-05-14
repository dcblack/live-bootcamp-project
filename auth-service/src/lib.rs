// This struct encapsulates our application-related logic.
use tower_http::services::ServeDir;
use axum::{response::Html, routing::get, Router, serve::Serve};

pub struct Application {
    server: Serve<Router, Router>,
    // address is exposed as a public field,
    // so we have access to it in tests.
    pub address: String,
}

impl Application {
    #[tokio::main]
    pub async fn build(address: &str) -> Result<Self, Box<dyn std::error::Error>> {
        // Move the Router definition from `main.rs` to here.
        // Also, remove the `hello` route.
        // We don't need it at this point!
        let router = Router::new()
            .nest_service("/", ServeDir::new("assets"));
        //  .route("/hello", get(hello_handler));


        let listener = tokio::net::TcpListener::bind(address).await?;
        let address = listener.local_addr()?.to_string();
        let server = axum::serve(listener, router);

        // Create a new Application instance and return it
        Application { server, address }.ok()
    }

    pub async fn run(self) -> Result<(), std::io::Error> {
        println!("listening on {}", &self.address);
        self.server.await
    }

}