use std::sync::Arc;
use tokio::sync::RwLock;
use reqwest::cookie::Jar;

use auth_service::{
  app_state::AppState,
  services::{
    hashmap_user_store::HashmapUserStore,
    hashset_banned_token_store::HashsetBannedTokenStore,
    hashmap_two_fa_code_store::HashmapTwoFACodeStore,
    mock_email_client::MockEmailClient,
  },
  utils::constants::test,
  Application,
};

use uuid::Uuid;

pub const GREAT_PASSWORD: &str = "Decent-Pa55word!";
pub const WRONG_PASSWORD: &str = "Another-Pa$$w0rd";
//pub const THIRD_PASSWORD: &str = "Alternate-Pa$$w0rd";
//pub const SPECIAL_CHARS: &str = "Li!1";
pub const SHORT_PASSWORD: &str = "_bCdef$12ee"; // with valid characters
pub const EMPTY_PASSWORD: &str = "";

pub struct TestApp {
  pub address: String,
  pub banned_token_store: Arc<RwLock<HashsetBannedTokenStore>>,
  pub two_fa_code_store: Arc<RwLock<HashmapTwoFACodeStore>>,
  pub email_client: Arc<RwLock<MockEmailClient>>,
  pub cookie_jar: Arc<Jar>,
  pub http_client: reqwest::Client,
}

impl TestApp {
  pub async fn new() -> Self {
    let user_store = Arc::new(RwLock::new(HashmapUserStore::default()));
    let banned_token_store = Arc::new(RwLock::new(HashsetBannedTokenStore::default()));
    let two_fa_code_store = Arc::new(RwLock::new(HashmapTwoFACodeStore::default()));
    let email_client = Arc::new(RwLock::new(MockEmailClient));
    let app_state = AppState::new(user_store, banned_token_store.clone(), two_fa_code_store.clone(), email_client.clone());

    let app = Application::build(app_state, test::APP_ADDRESS)
      .await
      .expect("Failed to build app");

    let address = format!("http://{}", app.address.clone());

    #[allow(clippy::let_underscore_future)]
    let _ = tokio::spawn(app.run());

    let cookie_jar = Arc::new(Jar::default());
    let http_client = reqwest::Client::builder()
      .cookie_provider(cookie_jar.clone())
      .build()
      .unwrap();

    Self {
      address,
      cookie_jar,
      banned_token_store,
      two_fa_code_store,
      email_client,
      http_client,
    }
  }

  pub async fn get_root(&self) -> reqwest::Response {
    self.http_client
        .get(&format!("{}/", &self.address))
        .send()
        .await
        .expect("Failed to execute request.")
  }

  pub async fn post_signup<Body>(&self, body: &Body) -> reqwest::Response
  where
    Body: serde::Serialize,
  {
    self.http_client
        .post(&format!("{}/signup", &self.address))
        .json(body)
        .send()
        .await
        .expect("Failed to execute request.")
  }

  pub async fn post_login<Body>(&self, body: &Body) -> reqwest::Response
  where
    Body: serde::Serialize,
  {
    self.http_client
        .post(&format!("{}/login", &self.address))
        .json(body)
        .send()
        .await
        .expect("Failed to execute request.")
  }

  pub async fn post_logout(&self) -> reqwest::Response {
    self.http_client
        .post(format!("{}/logout", &self.address))
        .send()
        .await
        .expect("Failed to execute request.")
  }

  pub async fn post_verify_token<Body>(&self, body: &Body) -> reqwest::Response
  where
    Body: serde::Serialize,
  {
    self.http_client
        .post(format!("{}/verify-token", &self.address))
        .json(body)
        .send()
        .await
        .expect("Failed to execute request.")
  }

  pub async fn post_verify_2fa<Body>(&self, body: &Body) -> reqwest::Response
  where
    Body: serde::Serialize,
  {
    self.http_client
        .post(format!("{}/verify-2fa", &self.address))
        .json(body)
        .send()
        .await
        .expect("Failed to execute request.")
  }
}

pub fn get_random_email() -> String {
  format!("{}@example.com", Uuid::new_v4())
}
