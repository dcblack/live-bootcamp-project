use auth_service::Application;

#[tokio::main]
async fn main() {
    let user_store = Application::app_state::UserStoreType::default();
    let app_state = Application::app_state::AppState::new(user_store);
    let app = Application::build( app_state, "0.0.0.0:3000")
        .await
        .expect("Failed to build app");

    app.run().await.expect("Failed to run app");
}
