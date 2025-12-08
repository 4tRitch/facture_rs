use facture_api::{auth::{app::App, input::RefreshInput, refresh_token::RefreshToken}, request::FactureRequest};

#[tokio::test]
async fn refresh_token() {
  let input = RefreshInput{
    app: App::new()
      .client_id("U4SaLXc3qw9NoKWMVp9K")
      .secret("CjNifhbaEMyaaoTg3U0m7W9FllSioOCZ"),

    refresh_token: "b6f52bc91411ca0895b474cf7c488200".to_string()
  };

  let response = RefreshToken.request(input).await.unwrap();

  println!("expires_in: {}",response.expires_in);
  println!("access_token: {}",response.access_token);
  println!("refresh_token: {}",response.refresh_token);
}
