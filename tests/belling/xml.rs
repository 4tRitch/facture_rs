use facture_api::{
  auth::{password::Password, utils::input::PasswordInput},
  core::{app::App, credentials::AppCredentials, request::FactureRequest}
};


#[tokio::test]
async fn password_auth() {
  let scopes = "timbrado sucursal facturacion cancelacion comprobante_recibido".to_string();

  let credentials = AppCredentials::new()
    .user("cl8122018@gmail.com")
    .password("12345678a");

  let app = App::new()
    .set_client_id("U4SaLXc3qw9NoKWMVp9K")
    .set_client_secret("CjNifhbaEMyaaoTg3U0m7W9FllSioOCZ")
    .set_credentials(credentials);

  let input = PasswordInput{
    app: app,
    scopes: scopes
  };

  let response = Password.request(input).await.unwrap();

  // let date = response.expires_in;
  // let access = response.access_token;
  // let refresh = response.refresh_token;

  println!("expires_in: {}",response.expires_in);
  println!("access_token: {}",response.access_token);
  println!("refresh_token: {}",response.refresh_token);
}
