use facture_rs::{auth::{refresh_token::RefreshToken, utils::input::RefreshInput}, core::{app::App, request::FactureRequest}};

#[tokio::test]
async fn refresh_token() {
  let input = RefreshInput{
    app: App::new()
      .set_client_id("U4SaLXc3qw9NoKWMVp9K")
      .set_client_secret("CjNifhbaEMyaaoTg3U0m7W9FllSioOCZ"),

    refresh_token: "b6f52bc91411ca0895b474cf7c488200".to_string()
  };

  let response = RefreshToken.request(input).await.unwrap();

  println!("expires_in: {}",response.expires_in);
  println!("access_token: {}",response.access_token);
  println!("refresh_token: {}",response.refresh_token);
}
