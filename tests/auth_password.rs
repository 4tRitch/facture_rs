use facture_api::{auth::{app::App, credentials::AuthCredentials, input::PasswordInput, password::Password}, request::FactureRequest};


// #[tokio::test]
async fn test_password_auth() {
  let scopes = "timbrado sucursal facturacion cancelacion comprobante_recibido";
  let input = PasswordInput{
    credentials: AuthCredentials::new()
      .user("cl8122018@gmail.com")
      .password("12345678a"),
    app: App::new()
      .client_id("U4SaLXc3qw9NoKWMVp9K")
      .secret("CjNifhbaEMyaaoTg3U0m7W9FllSioOCZ"),
    scopes: scopes.to_string()
  };

  let response = Password.request(input).await.unwrap();

  let date = response.expires_in;
  let access = response.access_token;
  let refresh = response.refresh_token;

  println!("expires_in: {date}");
  println!("access_token: {access}");
  println!("refresh_token: {refresh}");
}
