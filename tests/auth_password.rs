use facture_api::auth::password::{AuthClient, Password};

#[tokio::test]
async fn test_password_auth() {
  let mut auth = Password::new();

  let client = AuthClient::new()
    .user("cl8122018@gmail.com")
    .password("12345678a")
    .client_id("U4SaLXc3qw9NoKWMVp9K")
    .secret("CjNifhbaEMyaaoTg3U0m7W9FllSioOCZ");

  let scopes = "timbrado sucursal facturacion cancelacion comprobante_recibido";
  let response = auth.password(client, scopes.to_string()).await.unwrap();

  let date = response.expires_in;
  let access = response.access_token;
  let refresh = response.refresh_token;

  println!("expires_in: {date}");
  println!("access_token: {access}");
  println!("refresh_token: {refresh}");
}
